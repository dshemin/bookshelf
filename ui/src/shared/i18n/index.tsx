import { initReactI18next } from "react-i18next";
import ruTrans from "./locales/ru.json";
import i18next from "i18next";

export const setupTranslation = (env: string = "development"): void => {
    i18next
        .use(initReactI18next)
        .init({
            lng: "ru",
            fallbackLng: "ru",
            debug: env !== "production",
            react: {
                useSuspense: true
            },
            interpolation: {
                escapeValue: false,
            },
            resources: {
                "ru": {
                    "translation": ruTrans,
                },
            },
        });
};
