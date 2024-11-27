import { type ClassValue, clsx } from "clsx";
import { cubicOut } from "svelte/easing";
import type { TransitionConfig } from "svelte/transition";
import { twMerge } from "tailwind-merge";

export function getRandomColor() {
  const colors = [
    "bg-blue-500",
    "bg-green-500",
    "bg-red-500",
    "bg-yellow-500",
    "bg-purple-500",
    "bg-teal-500",
    "bg-indigo-500",
    "bg-orange-500",
    "bg-gray-500",
    "bg-lime-500",
    "bg-cyan-500",
    "bg-emerald-500",
    "bg-fuchsia-500",
    "bg-rose-500",
    "bg-violet-500",
    "bg-amber-500",
    "bg-blue-400",
    "bg-green-400",
    "bg-red-400",
    "bg-yellow-400",
    "bg-purple-400",
    "bg-teal-400",
    "bg-indigo-400",
    "bg-orange-400",
    "bg-gray-400",
    "bg-lime-400",
    "bg-cyan-400",
    "bg-emerald-400",
    "bg-fuchsia-400",
    "bg-rose-400",
    "bg-violet-400",
    "bg-amber-400",
    "bg-blue-600",
    "bg-green-600",
    "bg-red-600",
    "bg-yellow-600",
    "bg-purple-600",
    "bg-teal-600",
    "bg-indigo-600",
    "bg-orange-600",
    "bg-gray-600",
    "bg-lime-600",
    "bg-cyan-600",
    "bg-emerald-600",
    "bg-fuchsia-600",
    "bg-rose-600",
    "bg-violet-600",
    "bg-amber-600",
    "bg-blue-700",
    "bg-green-700",
    "bg-red-700",
    "bg-yellow-700",
    "bg-purple-700",
    "bg-teal-700",
    "bg-indigo-700",
    "bg-orange-700",
    "bg-gray-700",
    "bg-lime-700",
    "bg-cyan-700",
    "bg-emerald-700",
    "bg-fuchsia-700",
    "bg-rose-700",
    "bg-violet-700",
    "bg-amber-700",
  ];

  return colors[Math.floor(Math.random() * colors.length)];
}

export function formatDate(
  dateString: string,
  locale: string = "en-US",
  options: Intl.DateTimeFormatOptions = {},
): string {
  const date = new Date(dateString);

  const defaultOptions: Intl.DateTimeFormatOptions = {
    year: "numeric",
    month: "long",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
    hour12: true,
  };

  const formatter = new Intl.DateTimeFormat(locale, {
    ...defaultOptions,
    ...options,
  });

  return formatter.format(date);
}

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

type FlyAndScaleParams = {
  y?: number;
  x?: number;
  start?: number;
  duration?: number;
};

export const flyAndScale = (
  node: Element,
  params: FlyAndScaleParams = { y: -8, x: 0, start: 0.95, duration: 150 },
): TransitionConfig => {
  const style = getComputedStyle(node);
  const transform = style.transform === "none" ? "" : style.transform;

  const scaleConversion = (
    valueA: number,
    scaleA: [number, number],
    scaleB: [number, number],
  ) => {
    const [minA, maxA] = scaleA;
    const [minB, maxB] = scaleB;

    const percentage = (valueA - minA) / (maxA - minA);
    const valueB = percentage * (maxB - minB) + minB;

    return valueB;
  };

  const styleToString = (
    style: Record<string, number | string | undefined>,
  ): string => {
    return Object.keys(style).reduce((str, key) => {
      if (style[key] === undefined) return str;
      return `${str}${key}:${style[key]};`;
    }, "");
  };

  return {
    duration: params.duration ?? 200,
    delay: 0,
    css: (t) => {
      const y = scaleConversion(t, [0, 1], [params.y ?? 5, 0]);
      const x = scaleConversion(t, [0, 1], [params.x ?? 0, 0]);
      const scale = scaleConversion(t, [0, 1], [params.start ?? 0.95, 1]);

      return styleToString({
        transform: `${transform} translate3d(${x}px, ${y}px, 0) scale(${scale})`,
        opacity: t,
      });
    },
    easing: cubicOut,
  };
};
