import { Languages } from "lucide-react";
import { useTranslation } from "react-i18next";

function LangButton() {
  const { i18n } = useTranslation();

  const changeLanguage = (lang: string) => {
    i18n.changeLanguage(lang);
  };

  const currentLang = i18n.language?.toUpperCase();

  return (
    <button
      onClick={() => changeLanguage(currentLang === "PT" ? "en" : "pt")}
      className="flex items-center justify-center gap-1 rounded-full h-9 bg-gray-700 px-3 text-white hover:bg-gray-600 transition cursor-pointer"
    >
      <Languages size={18} />

      <span className="flex items-center gap-1 text-sm font-medium">
        {currentLang}
      </span>
    </button>
  );
}

export default LangButton;