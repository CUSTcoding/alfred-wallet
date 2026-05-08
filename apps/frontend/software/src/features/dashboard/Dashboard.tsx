import { useState } from "react";
import { motion } from "framer-motion";
import {
  Home,
  Network,
  Shield,
  Settings,
  Wallet,
  ChevronsLeft,
  ChevronsRight,
  User,
} from "lucide-react";
import type { LucideIcon } from "lucide-react";
import HomeComponent from "../../components/layout/HomeComponent";
import NotFound from "../../components/layout/NotFound";
import ProfileComponent from "../../components/layout/ProfileComponent";
import NetworkComponent  from "../../components/layout/NetworkComponent";
import PrivacyComponent from "../../components/layout/PrivacyComponent";
import SettingsComponent from "../../components/layout/SettingsComponent";

type PageKey = "home" | "user" | "network" | "privacy" | "settings";

function Dashboard() {

  const [expanded, setExpanded] = useState(true);

  const [active, setActive] = useState<PageKey>("home");

  const menu: {
    id: PageKey;
    label: string;
    icon: LucideIcon;
    component: React.ComponentType;
  }[] = [
    { id: "home", label: "Home", icon: Home, component: HomeComponent },
    { id: "user", label: "Profile", icon: User, component: ProfileComponent },
    { id: "network", label: "Network", icon: Network, component: NetworkComponent },
    { id: "privacy", label: "Privacy", icon: Shield, component: PrivacyComponent },
  ];

  const pages = {
    home: HomeComponent,
    user: ProfileComponent,
    network: NetworkComponent,
    privacy: PrivacyComponent,
    settings: SettingsComponent,
  };
     

  const ActiveComponent = pages[active] || NotFound;
  return (
    <main className="w-dvw h-dvh flex items-center justify-center bg-zinc-100">


      <motion.aside
        animate={{ width: expanded ? 240 : 80 }}
        transition={{ duration: 0.25, ease: "easeInOut" }}
        className="relative h-[98%] bg-zinc-900 text-white m-2 flex flex-col justify-between py-10 rounded-2xl"
      >


        <div>

  
          <div className="flex items-center justify-center gap-3 px-4 py-3">
            <Wallet />
            {expanded && (
              <motion.span
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                className="font-bold"
              >
                Alfred Wallet
              </motion.span>
            )}
          </div>

          <nav className="mt-6 flex flex-col  gap-1 px-2">
            {menu.map((item) => {
              const Icon = item.icon;
              const isActive = active === item.id;

              return (
                <button
                  key={item.id}
                  onClick={() => setActive(item.id)}
                  className={`flex items-center  gap-3 px-3 py-3 rounded-md transition
                     ${expanded ? "justify-start gap-3" : "justify-center"}
                    ${isActive ? "bg-indigo-600" : "hover:bg-zinc-800"}`}
                >
                  <Icon size={20} />

                  {/* TEXT ANIMADO */}
                  {expanded && (
                    <motion.span
                      initial={{ opacity: 0, x: -5 }}
                      animate={{ opacity: 1, x: 0 }}
                      className="text-sm"
                    >
                      {item.label}
                    </motion.span>
                  )}
                </button>
              );
            })}
          </nav>
        </div>


        <div className="px-2 border-t border-zinc-800 pt-3">

          <div className="flex items-center gap-3 px-2 py-3">

            <div className="w-10 h-10 rounded-full bg-indigo-500 flex items-center justify-center font-bold">
              A
            </div>

            {expanded && (
              <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                className="flex flex-col leading-tight"
              >
                <span className="text-sm font-semibold">Alfred</span>
                <span className="text-xs text-zinc-400">online</span>
              </motion.div>
            )}
          </div>

          
          <button
            onClick={() => setActive("settings")}
            className={`flex items-center justify-center gap-3 px-3 py-3 rounded-md w-full transition
              ${active === "settings" ? "bg-indigo-600" : "hover:bg-zinc-800"}
              ${expanded ? "justify-start gap-3" : "justify-center"}`}
          >
            <Settings size={20} />

            {expanded && (
              <motion.span
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                className="text-sm"
              >
                Settings
              </motion.span>
            )}
          </button>
        </div>

        <button
        onClick={() => setExpanded(!expanded)}
        className="absolute top-6 -right-3 
        bg-zinc-800 border border-zinc-700
        text-zinc-400 hover:text-white 
        p-2 rounded-full transition shadow-md cursor-pointer"
        >
        {expanded ? (
            <ChevronsLeft size={18} />
        ) : (
            <ChevronsRight size={18} />
        )}
        </button>

      </motion.aside>

      

    
      <div className="flex-1 p-4  h-full">
        {ActiveComponent ? (
          <ActiveComponent />
        ) : (
          <NotFound />
        )}
      </div>

    </main>
  );
}

export default Dashboard;