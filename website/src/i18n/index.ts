import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import dayjs from "dayjs";
import "dayjs/locale/uk";
import en from "./en.json";
import uk from "./uk.json";

const savedLang = localStorage.getItem("lumirum-lang") ?? "en";

i18n.use(initReactI18next).init({
  resources: {
    en: { translation: en },
    uk: { translation: uk },
  },
  lng: savedLang,
  fallbackLng: "en",
  interpolation: { escapeValue: false },
});

dayjs.locale(savedLang);

i18n.on("languageChanged", (lang) => {
  dayjs.locale(lang);
});

export default i18n;
