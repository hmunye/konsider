export function getCurrentTimeFormatted() {
  const now = new Date();
  const options = {
    weekday: "long" as const,
    year: "numeric" as const,
    month: "long" as const,
    day: "numeric" as const,
    hour: "numeric" as const,
    minute: "numeric" as const,
    hour12: true,
  };

  const formattedDate = now.toLocaleDateString("en-US", options);
  return `${formattedDate}`;
}
