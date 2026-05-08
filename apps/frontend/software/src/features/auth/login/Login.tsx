import { useState } from "react";
import { useTranslation } from "react-i18next";

function RestoreWallet() {
  const [type, setType] = useState<12 | 24>(12);
  const [seed, setSeed] = useState<string[]>(Array(12).fill(""));
  const [loading, setLoading] = useState(false);
  const [step, setStep] = useState(1);

  const handleTypeChange = (newType: 12 | 24) => {
    setType(newType);
    setSeed(Array(newType).fill(""));
  };

  const handleChange = (value: string, index: number) => {
    const newSeed = [...seed];
    newSeed[index] = value;
    setSeed(newSeed);
  };

  const handleRestore = () => {
    setLoading(true);

    setTimeout(() => {
      setLoading(false);
      setStep(2);
    }, 2500);
  };

   const { t } = useTranslation();

  return (
    <div className="h-dvh w-dvw flex items-center justify-center bg-black text-white px-6">

      <div className="w-full max-w-md bg-gray-900 rounded-2xl p-6 flex flex-col gap-6">

        {/* HEADER */}
        <div className="text-center">
          <h1 className="text-2xl font-bold">{t("restore.title")}</h1>
          <p className="text-gray-400 text-sm mt-1">
            {t("restore.subtitle")}
          </p>
        </div>

        {/* STEP 1 */}
        {step === 1 && (
          <>
            {/* TYPE SELECTOR */}
            <div className="flex justify-center gap-4">
              <button
                onClick={() => handleTypeChange(12)}
                className={`px-4 py-1 rounded-xl ${
                  type === 12 ? "bg-green-500" : "bg-gray-700"
                }`}
              >
                {t("restore.type.12")}
              </button>

              <button
                onClick={() => handleTypeChange(24)}
                className={`px-4 py-1 rounded-xl ${
                  type === 24 ? "bg-green-500" : "bg-gray-700"
                }`}
              >
                {t("restore.type.24")}
              </button>
            </div>

            {/* GRID INPUTS */}
            <div className="grid grid-cols-4 gap-2">
              {seed.map((word, index) => (
                <input
                  key={index}
                  value={word}
                  onChange={(e) => handleChange(e.target.value, index)}
                  placeholder={`${index + 1}`}
                  className="p-2 text-sm rounded-lg bg-gray-800 text-white  outline-none text-center"
                />
              ))}
            </div>

            <button
              onClick={handleRestore}
              disabled={seed.some((w) => !w)}
              className="bg-green-500 hover:bg-green-600 transition py-2 rounded-xl font-semibold disabled:opacity-40"
            >
               {t("restore.button.restore")}
            </button>

            <p className="text-xs text-gray-500 text-center">
              {t("restore.security")}
            </p>
          </>
        )}

        {/* STEP 2 - LOADING */}
        {loading && (
          <div className="text-center space-y-3">
            <div className="animate-spin w-8 h-8 border-2 border-green-500 border-t-transparent rounded-full mx-auto"></div>

            <p className="text-gray-300">
              Restoring on-chain wallet...
            </p>

            <p className="text-xs text-gray-500">
              Detecting Taproot (BIP86) structure
            </p>
          </div>
        )}

        {/* STEP 3 - SUCCESS */}
        {step === 2 && !loading && (
          <div className="text-center space-y-4">
            <div className="text-green-400 text-xl">✔ Wallet Restored</div>

            <div className="text-sm text-gray-400">
              Taproot wallet detected<br />
              Lightning layer initializing...
            </div>

            <button className="bg-blue-500 hover:bg-blue-600 transition py-2 px-4 rounded-xl">
              Go to Dashboard
            </button>
          </div>
        )}

      </div>
    </div>
  );
}

export default RestoreWallet;