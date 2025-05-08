import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

// 自动加载vue等API
import AutoImport from "unplugin-auto-import/vite";
// import { dirResolver } from 'vite-auto-import-resolvers' // 引入自定义解析器 比如store
import { dirResolver, DirResolverHelper } from "vite-auto-import-resolvers";
// 组件全自动按需引入
import Components from "unplugin-vue-components/vite"; // 组件文件夹无法自动引入
import {
	NaiveUiResolver,
	VueUseComponentsResolver,
} from "unplugin-vue-components/resolvers";
import path from "path"; //
// https://vitejs.dev/config/
export default defineConfig(async () => ({
	build: {
		outDir: ".vite",
	},
	plugins: [
		vue(),
		DirResolverHelper(),
		AutoImport({
			dirs: ["./composables/*.ts", "./stores/*.ts"],
			dts: "auto-imports.d.ts",
			injectAtEnd: false,
			imports: [
				"vue",
				"pinia",
				"vue-router",
				"vue-i18n",
				{
					"@tauri-apps/api/core": ["invoke"],
				},
			],
			vueTemplate: true, // 开启后vue模版中也可以使用，否则模版中undefined
			// resolvers: [dirResolver({ srcAlias: "././", target: "stores" })],
		}),
		Components({
			dirs: ["components"], // 指定组件文件夹
			resolvers: [NaiveUiResolver(), VueUseComponentsResolver()],
			dts: true,
		}),
	],
	// 添加路径别名
	resolve: {
		alias: {
			"@": path.resolve(__dirname, "./src"), // 设置当前路径别名，用于图标识别及引入
		},
	},
	// Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
	//
	// 1. prevent vite from obscuring rust errors
	clearScreen: false,
	// 2. tauri expects a fixed port, fail if that port is not available
	server: {
		port: 1420,
		strictPort: true,
		watch: {
			// 3. tell vite to ignore watching `src-tauri`
			ignored: ["**/src-tauri/**"],
		},
	},
}));
