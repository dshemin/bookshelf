import { PluginOption, defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tsconfigPaths from "vite-tsconfig-paths";
import { viteStaticCopy } from "vite-plugin-static-copy";

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => {
    let plugins: PluginOption[] = [
        react(),
        tsconfigPaths(),
    ];

    if (mode === "development") {
        plugins = [
            ...plugins,
            viteStaticCopy({
                targets: [
                    {
                        src: "data/sample.pdf",
                        dest: "/",
                    },
                ],
            }),
        ];
    }

    return {
        plugins,
        envPrefix: "BS_",
    };
});
