import { getAllReleases } from "./releases";

(async () => {
  const releases = await getAllReleases({ withPrereleases: true });

  console.log(releases);
})();
