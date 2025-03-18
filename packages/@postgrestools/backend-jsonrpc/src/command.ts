/**
 * Gets the path of the binary for the current platform
 *
 * @returns Filesystem path to the binary, or null if no prebuilt distribution exists for the current platform
 */
export function getCommand(): string | null {
	const { platform, arch } = process;

	type PlatformPaths = {
		[P in NodeJS.Platform]?: {
			[A in NodeJS.Architecture]?: string;
		};
	};

	const PLATFORMS: PlatformPaths = {
		win32: {
			x64: "@postgrestools/cli-win32-x64/postgrestools.exe",
			arm64: "@postgrestools/cli-win32-arm64/postgrestools.exe",
		},
		darwin: {
			x64: "@postgrestools/cli-darwin-x64/postgrestools",
			arm64: "@postgrestools/cli-darwin-arm64/postgrestools",
		},
		linux: {
			x64: "@postgrestools/cli-linux-x64/postgrestools",
			arm64: "@postgrestools/cli-linux-arm64/postgrestools",
		},
	};

	const binPath = PLATFORMS?.[platform]?.[arch];
	if (!binPath) {
		return null;
	}

	return require.resolve(binPath);
}
