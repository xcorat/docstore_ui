
// this file is generated — do not edit it


/// <reference types="@sveltejs/kit" />

/**
 * Environment variables [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env`. Like [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), this module cannot be imported into client-side code. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 * 
 * _Unlike_ [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), the values exported from this module are statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * ```ts
 * import { API_KEY } from '$env/static/private';
 * ```
 * 
 * Note that all environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * 
 * ```
 * MY_FEATURE_FLAG=""
 * ```
 * 
 * You can override `.env` values from the command line like so:
 * 
 * ```bash
 * MY_FEATURE_FLAG="enabled" npm run dev
 * ```
 */
declare module '$env/static/private' {
	export const SHELL: string;
	export const npm_command: string;
	export const SESSION_MANAGER: string;
	export const WINDOWID: string;
	export const COLORTERM: string;
	export const XDG_CONFIG_DIRS: string;
	export const LESS: string;
	export const XDG_SESSION_PATH: string;
	export const NVM_INC: string;
	export const XDG_MENU_PREFIX: string;
	export const TERM_PROGRAM_VERSION: string;
	export const MACHTYPE: string;
	export const npm_package_devDependencies_prettier_plugin_tailwindcss: string;
	export const G_BROKEN_FILENAMES: string;
	export const HISTSIZE: string;
	export const HOSTNAME: string;
	export const ICEAUTHORITY: string;
	export const FROM_HEADER: string;
	export const LANGUAGE: string;
	export const MINICOM: string;
	export const NODE: string;
	export const npm_package_devDependencies_autoprefixer: string;
	export const npm_package_devDependencies_tailwindcss: string;
	export const npm_package_scripts_check_watch: string;
	export const AUDIODRIVER: string;
	export const JRE_HOME: string;
	export const CPU: string;
	export const SHELL_SESSION_ID: string;
	export const DESKTOP_SESSION: string;
	export const GTK_RC_FILES: string;
	export const NO_AT_BRIDGE: string;
	export const GPG_TTY: string;
	export const XDG_SEAT: string;
	export const PWD: string;
	export const NIX_PROFILES: string;
	export const QEMU_AUDIO_DRV: string;
	export const npm_package_devDependencies_vite: string;
	export const LOGNAME: string;
	export const XDG_SESSION_DESKTOP: string;
	export const npm_package_devDependencies__testing_library_svelte: string;
	export const XDG_SESSION_TYPE: string;
	export const npm_package_scripts_test_coverage: string;
	export const MANPATH: string;
	export const SYSTEMD_EXEC_PID: string;
	export const npm_package_scripts_build: string;
	export const XAUTHORITY: string;
	export const npm_package_devDependencies_prettier: string;
	export const LS_OPTIONS: string;
	export const VSCODE_GIT_ASKPASS_NODE: string;
	export const XKEYSYMDB: string;
	export const XKB_DEFAULT_MODEL: string;
	export const GTK2_RC_FILES: string;
	export const XNLSPATH: string;
	export const HOME: string;
	export const LANG: string;
	export const npm_package_devDependencies_typescript: string;
	export const LS_COLORS: string;
	export const XDG_CURRENT_DESKTOP: string;
	export const KONSOLE_DBUS_SERVICE: string;
	export const npm_package_version: string;
	export const PYTHONSTARTUP: string;
	export const npm_package_devDependencies_tailwind_merge: string;
	export const WAYLAND_DISPLAY: string;
	export const NIX_SSL_CERT_FILE: string;
	export const npm_package_devDependencies__vitest_coverage_v8: string;
	export const KONSOLE_DBUS_SESSION: string;
	export const PROFILEHOME: string;
	export const OSTYPE: string;
	export const GIT_ASKPASS: string;
	export const XDG_SEAT_PATH: string;
	export const npm_package_dependencies_svelte_file_dropzone: string;
	export const LESS_ADVANCED_PREPROCESSOR: string;
	export const INVOCATION_ID: string;
	export const npm_package_devDependencies_prettier_plugin_svelte: string;
	export const KONSOLE_VERSION: string;
	export const MANAGERPID: string;
	export const INIT_CWD: string;
	export const CHROME_DESKTOP: string;
	export const npm_package_scripts_format: string;
	export const KDE_SESSION_UID: string;
	export const npm_package_scripts_preview: string;
	export const npm_lifecycle_script: string;
	export const NVM_DIR: string;
	export const VSCODE_GIT_ASKPASS_EXTRA_ARGS: string;
	export const QV4_GC_TIMELIMIT: string;
	export const XKB_DEFAULT_LAYOUT: string;
	export const npm_package_devDependencies__sveltejs_vite_plugin_svelte: string;
	export const npm_package_devDependencies_svelte_check: string;
	export const LESSCLOSE: string;
	export const npm_package_devDependencies_tailwind_variants: string;
	export const XDG_SESSION_CLASS: string;
	export const TERM: string;
	export const npm_package_name: string;
	export const G_FILENAME_ENCODING: string;
	export const npm_package_devDependencies_jsdom: string;
	export const HOST: string;
	export const XAUTHLOCALHOSTNAME: string;
	export const LESSOPEN: string;
	export const npm_package_type: string;
	export const USER: string;
	export const npm_config_frozen_lockfile: string;
	export const npm_package_devDependencies_vitest: string;
	export const VSCODE_GIT_IPC_HANDLE: string;
	export const COLORFGBG: string;
	export const QT_WAYLAND_RECONNECT: string;
	export const KDE_SESSION_VERSION: string;
	export const MORE: string;
	export const CSHEDIT: string;
	export const DISPLAY: string;
	export const npm_lifecycle_event: string;
	export const SHLVL: string;
	export const NVM_CD_FLAGS: string;
	export const WINDOWMANAGER: string;
	export const PAGER: string;
	export const CVS_RSH: string;
	export const XDG_VTNR: string;
	export const XDG_SESSION_ID: string;
	export const npm_package_devDependencies_clsx: string;
	export const npm_config_user_agent: string;
	export const npm_package_scripts_lint: string;
	export const PNPM_SCRIPT_SRC_DIR: string;
	export const npm_execpath: string;
	export const npm_package_devDependencies__sveltejs_adapter_auto: string;
	export const npm_package_devDependencies_svelte: string;
	export const npm_package_scripts_test: string;
	export const LC_CTYPE: string;
	export const XDG_RUNTIME_DIR: string;
	export const npm_package_devDependencies_bits_ui: string;
	export const NODE_PATH: string;
	export const DEBUGINFOD_URLS: string;
	export const npm_package_scripts_dev: string;
	export const VSCODE_GIT_ASKPASS_MAIN: string;
	export const QT_AUTO_SCREEN_SCALE_FACTOR: string;
	export const JOURNAL_STREAM: string;
	export const MANPATHISSET: string;
	export const XCURSOR_THEME: string;
	export const XDG_DATA_DIRS: string;
	export const npm_package_scripts_check: string;
	export const GDK_BACKEND: string;
	export const KDE_FULL_SESSION: string;
	export const CONFIG_SITE: string;
	export const VENDOR: string;
	export const PATH: string;
	export const npm_config_node_gyp: string;
	export const npm_package_devDependencies__sveltejs_kit: string;
	export const ORIGINAL_XDG_CURRENT_DESKTOP: string;
	export const DBUS_SESSION_BUS_ADDRESS: string;
	export const PROFILEREAD: string;
	export const KDE_APPLICATIONS_AS_SCOPE: string;
	export const MAIL: string;
	export const NVM_BIN: string;
	export const npm_config_registry: string;
	export const HOSTTYPE: string;
	export const NODE_VERSION: string;
	export const XKB_DEFAULT_OPTIONS: string;
	export const npm_node_execpath: string;
	export const npm_config_engine_strict: string;
	export const LESSKEY: string;
	export const OLDPWD: string;
	export const TERM_PROGRAM: string;
	export const KONSOLE_DBUS_WINDOW: string;
	export const NODE_ENV: string;
}

/**
 * Similar to [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private), except that it only includes environment variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Values are replaced statically at build time.
 * 
 * ```ts
 * import { PUBLIC_BASE_URL } from '$env/static/public';
 * ```
 */
declare module '$env/static/public' {
	
}

/**
 * This module provides access to runtime environment variables, as defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/main/packages/adapter-node) (or running [`vite preview`](https://svelte.dev/docs/kit/cli)), this is equivalent to `process.env`. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 * 
 * This module cannot be imported into client-side code.
 * 
 * Dynamic environment variables cannot be used during prerendering.
 * 
 * ```ts
 * import { env } from '$env/dynamic/private';
 * console.log(env.DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 * 
 * > In `dev`, `$env/dynamic` always includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 */
declare module '$env/dynamic/private' {
	export const env: {
		SHELL: string;
		npm_command: string;
		SESSION_MANAGER: string;
		WINDOWID: string;
		COLORTERM: string;
		XDG_CONFIG_DIRS: string;
		LESS: string;
		XDG_SESSION_PATH: string;
		NVM_INC: string;
		XDG_MENU_PREFIX: string;
		TERM_PROGRAM_VERSION: string;
		MACHTYPE: string;
		npm_package_devDependencies_prettier_plugin_tailwindcss: string;
		G_BROKEN_FILENAMES: string;
		HISTSIZE: string;
		HOSTNAME: string;
		ICEAUTHORITY: string;
		FROM_HEADER: string;
		LANGUAGE: string;
		MINICOM: string;
		NODE: string;
		npm_package_devDependencies_autoprefixer: string;
		npm_package_devDependencies_tailwindcss: string;
		npm_package_scripts_check_watch: string;
		AUDIODRIVER: string;
		JRE_HOME: string;
		CPU: string;
		SHELL_SESSION_ID: string;
		DESKTOP_SESSION: string;
		GTK_RC_FILES: string;
		NO_AT_BRIDGE: string;
		GPG_TTY: string;
		XDG_SEAT: string;
		PWD: string;
		NIX_PROFILES: string;
		QEMU_AUDIO_DRV: string;
		npm_package_devDependencies_vite: string;
		LOGNAME: string;
		XDG_SESSION_DESKTOP: string;
		npm_package_devDependencies__testing_library_svelte: string;
		XDG_SESSION_TYPE: string;
		npm_package_scripts_test_coverage: string;
		MANPATH: string;
		SYSTEMD_EXEC_PID: string;
		npm_package_scripts_build: string;
		XAUTHORITY: string;
		npm_package_devDependencies_prettier: string;
		LS_OPTIONS: string;
		VSCODE_GIT_ASKPASS_NODE: string;
		XKEYSYMDB: string;
		XKB_DEFAULT_MODEL: string;
		GTK2_RC_FILES: string;
		XNLSPATH: string;
		HOME: string;
		LANG: string;
		npm_package_devDependencies_typescript: string;
		LS_COLORS: string;
		XDG_CURRENT_DESKTOP: string;
		KONSOLE_DBUS_SERVICE: string;
		npm_package_version: string;
		PYTHONSTARTUP: string;
		npm_package_devDependencies_tailwind_merge: string;
		WAYLAND_DISPLAY: string;
		NIX_SSL_CERT_FILE: string;
		npm_package_devDependencies__vitest_coverage_v8: string;
		KONSOLE_DBUS_SESSION: string;
		PROFILEHOME: string;
		OSTYPE: string;
		GIT_ASKPASS: string;
		XDG_SEAT_PATH: string;
		npm_package_dependencies_svelte_file_dropzone: string;
		LESS_ADVANCED_PREPROCESSOR: string;
		INVOCATION_ID: string;
		npm_package_devDependencies_prettier_plugin_svelte: string;
		KONSOLE_VERSION: string;
		MANAGERPID: string;
		INIT_CWD: string;
		CHROME_DESKTOP: string;
		npm_package_scripts_format: string;
		KDE_SESSION_UID: string;
		npm_package_scripts_preview: string;
		npm_lifecycle_script: string;
		NVM_DIR: string;
		VSCODE_GIT_ASKPASS_EXTRA_ARGS: string;
		QV4_GC_TIMELIMIT: string;
		XKB_DEFAULT_LAYOUT: string;
		npm_package_devDependencies__sveltejs_vite_plugin_svelte: string;
		npm_package_devDependencies_svelte_check: string;
		LESSCLOSE: string;
		npm_package_devDependencies_tailwind_variants: string;
		XDG_SESSION_CLASS: string;
		TERM: string;
		npm_package_name: string;
		G_FILENAME_ENCODING: string;
		npm_package_devDependencies_jsdom: string;
		HOST: string;
		XAUTHLOCALHOSTNAME: string;
		LESSOPEN: string;
		npm_package_type: string;
		USER: string;
		npm_config_frozen_lockfile: string;
		npm_package_devDependencies_vitest: string;
		VSCODE_GIT_IPC_HANDLE: string;
		COLORFGBG: string;
		QT_WAYLAND_RECONNECT: string;
		KDE_SESSION_VERSION: string;
		MORE: string;
		CSHEDIT: string;
		DISPLAY: string;
		npm_lifecycle_event: string;
		SHLVL: string;
		NVM_CD_FLAGS: string;
		WINDOWMANAGER: string;
		PAGER: string;
		CVS_RSH: string;
		XDG_VTNR: string;
		XDG_SESSION_ID: string;
		npm_package_devDependencies_clsx: string;
		npm_config_user_agent: string;
		npm_package_scripts_lint: string;
		PNPM_SCRIPT_SRC_DIR: string;
		npm_execpath: string;
		npm_package_devDependencies__sveltejs_adapter_auto: string;
		npm_package_devDependencies_svelte: string;
		npm_package_scripts_test: string;
		LC_CTYPE: string;
		XDG_RUNTIME_DIR: string;
		npm_package_devDependencies_bits_ui: string;
		NODE_PATH: string;
		DEBUGINFOD_URLS: string;
		npm_package_scripts_dev: string;
		VSCODE_GIT_ASKPASS_MAIN: string;
		QT_AUTO_SCREEN_SCALE_FACTOR: string;
		JOURNAL_STREAM: string;
		MANPATHISSET: string;
		XCURSOR_THEME: string;
		XDG_DATA_DIRS: string;
		npm_package_scripts_check: string;
		GDK_BACKEND: string;
		KDE_FULL_SESSION: string;
		CONFIG_SITE: string;
		VENDOR: string;
		PATH: string;
		npm_config_node_gyp: string;
		npm_package_devDependencies__sveltejs_kit: string;
		ORIGINAL_XDG_CURRENT_DESKTOP: string;
		DBUS_SESSION_BUS_ADDRESS: string;
		PROFILEREAD: string;
		KDE_APPLICATIONS_AS_SCOPE: string;
		MAIL: string;
		NVM_BIN: string;
		npm_config_registry: string;
		HOSTTYPE: string;
		NODE_VERSION: string;
		XKB_DEFAULT_OPTIONS: string;
		npm_node_execpath: string;
		npm_config_engine_strict: string;
		LESSKEY: string;
		OLDPWD: string;
		TERM_PROGRAM: string;
		KONSOLE_DBUS_WINDOW: string;
		NODE_ENV: string;
		[key: `PUBLIC_${string}`]: undefined;
		[key: `${string}`]: string | undefined;
	}
}

/**
 * Similar to [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), but only includes variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Note that public dynamic environment variables must all be sent from the server to the client, causing larger network requests — when possible, use `$env/static/public` instead.
 * 
 * Dynamic environment variables cannot be used during prerendering.
 * 
 * ```ts
 * import { env } from '$env/dynamic/public';
 * console.log(env.PUBLIC_DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 */
declare module '$env/dynamic/public' {
	export const env: {
		[key: `PUBLIC_${string}`]: string | undefined;
	}
}
