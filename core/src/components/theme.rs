#[derive(PartialEq, Debug)]
pub enum ColorStyle {
    Primary,
    Warning,
    Danger,
    Success,
    Info,
    Default,
}

impl Default for ColorStyle {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(PartialEq, Debug)]
pub enum TypeStyle {
    Primary,
    Warning,
    Danger,
    Success,
    Info,
    Default,
}

impl Default for TypeStyle {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(PartialEq, Debug)]
pub enum SizeStyle {
    Xs,
    Sm,
    Base,
    Lg,
    Xl,
    Xl2,
    Xl3,
    Xl4,
    Xl5,
    Xl6,
    Xl7,
    Xl8,
    Xl9,
}

impl Default for SizeStyle {
    fn default() -> Self {
        Self::Base
    }
}

#[derive(PartialEq, Debug)]
pub enum Rounded {
    None,
    Sm,
    Rounded,
    Lg,
    Xl,
    Xl2,
    Xl3,
    Full,
}

impl Default for Rounded {
    fn default() -> Self {
        Self::None
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Number {
    N50,
    N100,
    N200,
    N300,
    N400,
    N500,
    N600,
    N700,
    N800,
    N900,
    N950,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Colors {
    Slate(Number),
    Gray(Number),
    Zinc(Number),
    Neutral(Number),
    Stone(Number),
    Red(Number),
    Orange(Number),
    Amber(Number),
    Yellow(Number),
    Lime(Number),
    Green(Number),
    Emerald(Number),
    Teal(Number),
    Cyan(Number),
    Sky(Number),
    Blue(Number),
    Indigo(Number),
    Violet(Number),
    Purple(Number),
    Fuchsia(Number),
    Pink(Number),
    Rose(Number),
}

impl Default for Colors {
    fn default() -> Self {
        Self::Slate(Number::N500)
    }
}

pub fn get_color(colors: Colors) -> String {
    let color = match colors {
        Colors::Slate(number) => match number {
            Number::N50 => "#f8fafc",
            Number::N100 => "#f1f5f9",
            Number::N200 => "#e2e8f0",
            Number::N300 => "#cbd5e1",
            Number::N400 => "#94a3b8",
            Number::N500 => "#64748b",
            Number::N600 => "#475569",
            Number::N700 => "#334155",
            Number::N800 => "#1e293b",
            Number::N900 => "#0f172a",
            Number::N950 => "#020617",
        },
        Colors::Gray(number) => match number {
            Number::N50 => "#f9fafb",
            Number::N100 => "#f3f4f6",
            Number::N200 => "#e5e7eb",
            Number::N300 => "#d1d5db",
            Number::N400 => "#9ca3af",
            Number::N500 => "#6b7280",
            Number::N600 => "#4b5563",
            Number::N700 => "#374151",
            Number::N800 => "#1f2937",
            Number::N900 => "#111827",
            Number::N950 => "#030712",
        },
        Colors::Zinc(number) => match number {
            Number::N50 => "#fafafa",
            Number::N100 => "#f4f4f5",
            Number::N200 => "#e4e4e7",
            Number::N300 => "#d4d4d8",
            Number::N400 => "#a1a1aa",
            Number::N500 => "#71717a",
            Number::N600 => "#52525b",
            Number::N700 => "#3f3f46",
            Number::N800 => "#27272a",
            Number::N900 => "#18181b",
            Number::N950 => "#09090b",
        },
        Colors::Neutral(number) => match number {
            Number::N50 => "#fafafa",
            Number::N100 => "#f5f5f5",
            Number::N200 => "#e5e5e5",
            Number::N300 => "#d4d4d4",
            Number::N400 => "#a3a3a3",
            Number::N500 => "#737373",
            Number::N600 => "#525252",
            Number::N700 => "#404040",
            Number::N800 => "#262626",
            Number::N900 => "#171717",
            Number::N950 => "#0a0a0a",
        },
        Colors::Stone(number) => match number {
            Number::N50 => "#fafaf9",
            Number::N100 => "#f5f5f4",
            Number::N200 => "#e7e5e4",
            Number::N300 => "#d6d3d1",
            Number::N400 => "#a8a29e",
            Number::N500 => "#78716c",
            Number::N600 => "#57534e",
            Number::N700 => "#44403c",
            Number::N800 => "#292524",
            Number::N900 => "#1c1917",
            Number::N950 => "#0c0a09",
        },
        Colors::Red(number) => match number {
            Number::N50 => "#fef2f2",
            Number::N100 => "#fee2e2",
            Number::N200 => "#fecaca",
            Number::N300 => "#fca5a5",
            Number::N400 => "#f87171",
            Number::N500 => "#ef4444",
            Number::N600 => "#dc2626",
            Number::N700 => "#b91c1c",
            Number::N800 => "#991b1b",
            Number::N900 => "#7f1d1d",
            Number::N950 => "#450a0a",
        },
        Colors::Orange(number) => match number {
            Number::N50 => "#fff7ed",
            Number::N100 => "#ffedd5",
            Number::N200 => "#fed7aa",
            Number::N300 => "#fdba74",
            Number::N400 => "#fb923c",
            Number::N500 => "#f97316",
            Number::N600 => "#ea580c",
            Number::N700 => "#c2410c",
            Number::N800 => "#9a3412",
            Number::N900 => "#7c2d12",
            Number::N950 => "#431407",
        },
        // Add the remaining Colorss...
        // For example, Amber, Yellow, Lime, etc.
        Colors::Amber(number) => match number {
            Number::N50 => "#fffbeb",
            Number::N100 => "#fef3c7",
            Number::N200 => "#fde68a",
            Number::N300 => "#fcd34d",
            Number::N400 => "#fbbf24",
            Number::N500 => "#f59e0b",
            Number::N600 => "#d97706",
            Number::N700 => "#b45309",
            Number::N800 => "#92400e",
            Number::N900 => "#78350f",
            Number::N950 => "#451a03",
        },
        Colors::Yellow(number) => match number {
            Number::N50 => "#fefce8",
            Number::N100 => "#fef9c3",
            Number::N200 => "#fef08a",
            Number::N300 => "#fde047",
            Number::N400 => "#facc15",
            Number::N500 => "#eab308",
            Number::N600 => "#ca8a04",
            Number::N700 => "#a16207",
            Number::N800 => "#854d0e",
            Number::N900 => "#713f12",
            Number::N950 => "#422006",
        },
        Colors::Lime(number) => match number {
            Number::N50 => "#f7fee7",
            Number::N100 => "#ecfccb",
            Number::N200 => "#d9f99d",
            Number::N300 => "#bef264",
            Number::N400 => "#a3e635",
            Number::N500 => "#84cc16",
            Number::N600 => "#65a30d",
            Number::N700 => "#4d7c0f",
            Number::N800 => "#3f6212",
            Number::N900 => "#365314",
            Number::N950 => "#1a2e05",
        },
        Colors::Green(number) => match number {
            Number::N50 => "#f0fdf4",
            Number::N100 => "#dcfce7",
            Number::N200 => "#bbf7d0",
            Number::N300 => "#86efac",
            Number::N400 => "#4ade80",
            Number::N500 => "#22c55e",
            Number::N600 => "#16a34a",
            Number::N700 => "#15803d",
            Number::N800 => "#166534",
            Number::N900 => "#14532d",
            Number::N950 => "#052e16",
        },
        Colors::Emerald(number) => match number {
            Number::N50 => "#ecfdf5",
            Number::N100 => "#d1fae5",
            Number::N200 => "#a7f3d0",
            Number::N300 => "#6ee7b7",
            Number::N400 => "#34d399",
            Number::N500 => "#10b981",
            Number::N600 => "#059669",
            Number::N700 => "#047857",
            Number::N800 => "#065f46",
            Number::N900 => "#064e3b",
            Number::N950 => "#022c22",
        },
        Colors::Teal(number) => match number {
            Number::N50 => "#f0fdfa",
            Number::N100 => "#ccfbf1",
            Number::N200 => "#99f6e4",
            Number::N300 => "#5eead4",
            Number::N400 => "#2dd4bf",
            Number::N500 => "#14b8a6",
            Number::N600 => "#0d9488",
            Number::N700 => "#0f766e",
            Number::N800 => "#115e59",
            Number::N900 => "#134e4a",
            Number::N950 => "#042f2e",
        },
        Colors::Cyan(number) => match number {
            Number::N50 => "#ecfeff",
            Number::N100 => "#cffafe",
            Number::N200 => "#a5f3fc",
            Number::N300 => "#67e8f9",
            Number::N400 => "#22d3ee",
            Number::N500 => "#06b6d4",
            Number::N600 => "#0891b2",
            Number::N700 => "#0e7490",
            Number::N800 => "#155e75",
            Number::N900 => "#164e63",
            Number::N950 => "#083344",
        },
        Colors::Sky(number) => match number {
            Number::N50 => "#f0f9ff",
            Number::N100 => "#e0f2fe",
            Number::N200 => "#bae6fd",
            Number::N300 => "#7dd3fc",
            Number::N400 => "#38bdf8",
            Number::N500 => "#0ea5e9",
            Number::N600 => "#0284c7",
            Number::N700 => "#0369a1",
            Number::N800 => "#075985",
            Number::N900 => "#0c4a6e",
            Number::N950 => "#082f49",
        },
        Colors::Blue(number) => match number {
            Number::N50 => "#eff6ff",
            Number::N100 => "#dbeafe",
            Number::N200 => "#bfdbfe",
            Number::N300 => "#93c5fd",
            Number::N400 => "#60a5fa",
            Number::N500 => "#3b82f6",
            Number::N600 => "#2563eb",
            Number::N700 => "#1d4ed8",
            Number::N800 => "#1e40af",
            Number::N900 => "#1e3a8a",
            Number::N950 => "#172554",
        },
        Colors::Indigo(number) => match number {
            Number::N50 => "#eef2ff",
            Number::N100 => "#e0e7ff",
            Number::N200 => "#c7d2fe",
            Number::N300 => "#a5b4fc",
            Number::N400 => "#818cf8",
            Number::N500 => "#6366f1",
            Number::N600 => "#4f46e5",
            Number::N700 => "#4338ca",
            Number::N800 => "#3730a3",
            Number::N900 => "#312e81",
            Number::N950 => "#1e1b4b",
        },
        Colors::Violet(number) => match number {
            Number::N50 => "#f5f3ff",
            Number::N100 => "#ede9fe",
            Number::N200 => "#ddd6fe",
            Number::N300 => "#c4b5fd",
            Number::N400 => "#a78bfa",
            Number::N500 => "#8b5cf6",
            Number::N600 => "#7c3aed",
            Number::N700 => "#6d28d9",
            Number::N800 => "#5b21b6",
            Number::N900 => "#4c1d95",
            Number::N950 => "#2e1065",
        },
        Colors::Purple(number) => match number {
            Number::N50 => "#faf5ff",
            Number::N100 => "#f3e8ff",
            Number::N200 => "#e9d5ff",
            Number::N300 => "#d8b4fe",
            Number::N400 => "#c084fc",
            Number::N500 => "#a855f7",
            Number::N600 => "#9333ea",
            Number::N700 => "#7e22ce",
            Number::N800 => "#6b21a8",
            Number::N900 => "#581c87",
            Number::N950 => "#3b0764",
        },
        Colors::Fuchsia(number) => match number {
            Number::N50 => "#fdf4ff",
            Number::N100 => "#fae8ff",
            Number::N200 => "#f5d0fe",
            Number::N300 => "#f0abfc",
            Number::N400 => "#e879f9",
            Number::N500 => "#d946ef",
            Number::N600 => "#c026d3",
            Number::N700 => "#a21caf",
            Number::N800 => "#86198f",
            Number::N900 => "#701a75",
            Number::N950 => "#4a044e",
        },
        Colors::Pink(number) => match number {
            Number::N50 => "#fdf2f8",
            Number::N100 => "#fce7f3",
            Number::N200 => "#fbcfe8",
            Number::N300 => "#f9a8d4",
            Number::N400 => "#f472b6",
            Number::N500 => "#ec4899",
            Number::N600 => "#db2777",
            Number::N700 => "#be185d",
            Number::N800 => "#9d174d",
            Number::N900 => "#831843",
            Number::N950 => "#500724",
        },
        Colors::Rose(number) => match number {
            Number::N50 => "#fff1f2",
            Number::N100 => "#ffe4e6",
            Number::N200 => "#fecdd3",
            Number::N300 => "#fda4af",
            Number::N400 => "#fb7185",
            Number::N500 => "#f43f5e",
            Number::N600 => "#e11d48",
            Number::N700 => "#be123c",
            Number::N800 => "#9f1239",
            Number::N900 => "#881337",
            Number::N950 => "#4c0519",
        },
    };
    return color.to_string();
}
