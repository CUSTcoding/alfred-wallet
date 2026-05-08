import { useTranslation } from "react-i18next";
import LangButton from "../components/ui/langButton";
import "swiper/css";
import "swiper/css/pagination";
import HeroSwiper from "../components/ui/swiper";
import { useNavigate } from "react-router-dom";

function AutoPage() {
  const { t } = useTranslation();
  const navigate = useNavigate();

  return (
    <main className="flex h-dvh w-dvw relative">
      <div className="swiper h-dvh absolute flex items-center justify-center w-dvw bg-red-400">
        <HeroSwiper/>
      </div>

      <div className="content w-1/2 xl:w-1/3 z-20 bg-gray-700/90 absolute right-4 top-4 rounded-3xl flex flex-col items-center justify-center gap-30 bottom-4">

        
        <header className="content text-center">
          <h1 className="text-white text-3xl font-bold">
            Alfred Wallet
          </h1>

          <h4 className="text-gray-300 text-2xl font-semibold">
            {t("slogan")}
          </h4>
        </header>

   
        <div className="listbutton flex flex-col gap-4 w-1/2 px-6">
          <button onClick={() => navigate("/register")} className="bg-green-500 text-white cursor-pointer py-2 rounded-xl p-2 ">
            {t("button.createWallet")}
          </button>

          <button onClick={() => navigate("/login")}  className="bg-gray-600 text-white py-2 cursor-pointer rounded-xl p-2 ">
            {t("button.restoreWallet")}
          </button>
        </div>

        <LangButton />

      </div>
    </main>
  );
}

export default AutoPage;