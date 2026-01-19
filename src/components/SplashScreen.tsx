import { useState, useEffect } from "react";
import sunLogo from "../assets/sun-logo.jpg";

interface SplashScreenProps {
  onComplete: () => void;
  minimumDisplayTime?: number;
}

export function SplashScreen({ onComplete, minimumDisplayTime = 2500 }: SplashScreenProps) {
  const [fadeOut, setFadeOut] = useState(false);

  useEffect(() => {
    const timer = setTimeout(() => {
      setFadeOut(true);
      // Wait for fade animation to complete before calling onComplete
      setTimeout(onComplete, 500);
    }, minimumDisplayTime);

    return () => clearTimeout(timer);
  }, [minimumDisplayTime, onComplete]);

  return (
    <div
      className={`fixed inset-0 z-50 flex flex-col items-center justify-center bg-[#0A0A0A] transition-opacity duration-500 ${
        fadeOut ? "opacity-0" : "opacity-100"
      }`}
    >
      {/* Sun Logo */}
      <div className="relative mb-8">
        <div className="w-64 h-64 md:w-80 md:h-80 rounded-full overflow-hidden shadow-2xl animate-pulse-slow">
          <img
            src={sunLogo}
            alt="4DA"
            className="w-full h-full object-cover"
          />
        </div>
        {/* Glow effect */}
        <div className="absolute inset-0 rounded-full bg-gradient-radial from-orange-500/20 via-transparent to-transparent blur-xl animate-glow" />
      </div>

      {/* Brand Name */}
      <h1 className="text-4xl md:text-5xl font-semibold text-white tracking-tight mb-3">
        4DA Home
      </h1>

      {/* Tagline */}
      <p className="text-lg md:text-xl text-gray-400 tracking-wide mb-8">
        The internet searches for you
      </p>

      {/* Loading indicator */}
      <div className="flex items-center gap-2">
        <div className="w-2 h-2 bg-orange-500 rounded-full animate-bounce" style={{ animationDelay: "0ms" }} />
        <div className="w-2 h-2 bg-orange-500 rounded-full animate-bounce" style={{ animationDelay: "150ms" }} />
        <div className="w-2 h-2 bg-orange-500 rounded-full animate-bounce" style={{ animationDelay: "300ms" }} />
      </div>

      {/* Version */}
      <p className="absolute bottom-6 text-xs text-gray-600">
        Version 0.1.0
      </p>
    </div>
  );
}
