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
			x64: "@pglt/cli-win32-x64/pglt.exe",
			arm64: "@pglt/cli-win32-arm64/pglt.exe",
		},
		darwin: {
			x64: "@pglt/cli-darwin-x64/pglt",
			arm64: "@pglt/cli-darwin-arm64/pglt",
		},
		linux: {
			x64: "@pglt/cli-linux-x64/pglt",
			arm64: "@pglt/cli-linux-arm64/pglt",
		},
	};

	const binPath = PLATFORMS?.[platform]?.[arch];
	if (!binPath) {
		return null;
	}

	return require.resolve(binPath);
}
