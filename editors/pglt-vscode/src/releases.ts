import { window } from "vscode";

export type Release = {
  tag_name: string;
  published_at: string;
  draft: boolean;
  prerelease: boolean;
};

export async function getAllReleases(opts: {
  withPrereleases: boolean;
}): Promise<Release[]> {
  let page = 1;
  let perPage = 100;
  let exhausted = false;

  const releases = [];

  while (!exhausted) {
    const queryParams = new URLSearchParams();

    queryParams.append("page", page.toString());
    queryParams.append("per_page", perPage.toString());

    const response = await fetch(
      "https://api.github.com/repos/supabase-community/postgres_lsp/releases",
      {
        method: "GET",
        headers: {
          "X-GitHub-Api-Version": "2022-11-28",
        },
      }
    );

    if (!response.ok) {
      const body = await response.json();
      window.showErrorMessage(
        `Could not fetch releases from GitHub! Received Status Code: ${response.status}, Body: ${body}`
      );
      return [];
    }

    const results = (await response.json()) as Release[];

    if (results.length === 0) {
      window.showErrorMessage(
        'No releases found on GitHub. Suggestion: Set "pglt.allowDownloadPrereleases" to `true` in your vscode settings.'
      );
      return [];
    }

    releases.push(...results);

    if (page > 30) {
      // sanity
      exhausted = true;
    } else if (results.length < perPage) {
      exhausted = true;
    } else {
      page++;
    }
  }

  return releases
    .filter(
      (r) =>
        !r.draft && // shouldn't be fetched without auth token, anyways
        (opts.withPrereleases || !r.prerelease)
    )
    .sort(
      (a, b) =>
        new Date(b.published_at).getTime() - new Date(a.published_at).getTime()
    );
}
