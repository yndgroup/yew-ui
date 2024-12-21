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
pub enum Levels {
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
    Slate(Levels),
    Gray(Levels),
    Zinc(Levels),
    Neutral(Levels),
    Stone(Levels),
    Red(Levels),
    Orange(Levels),
    Amber(Levels),
    Yellow(Levels),
    Lime(Levels),
    Green(Levels),
    Emerald(Levels),
    Teal(Levels),
    Cyan(Levels),
    Sky(Levels),
    Blue(Levels),
    Indigo(Levels),
    Violet(Levels),
    Purple(Levels),
    Fuchsia(Levels),
    Pink(Levels),
    Rose(Levels),
    Custom(String)
}

impl Default for Colors {
    fn default() -> Self {
        Self::Slate(Levels::N500)
    }
}

pub fn get_color(colors: Colors) -> String {
    let color = match colors {
        Colors::Slate(number) => match number {
            Levels::N50 => "#f8fafc",
            Levels::N100 => "#f1f5f9",
            Levels::N200 => "#e2e8f0",
            Levels::N300 => "#cbd5e1",
            Levels::N400 => "#94a3b8",
            Levels::N500 => "#64748b",
            Levels::N600 => "#475569",
            Levels::N700 => "#334155",
            Levels::N800 => "#1e293b",
            Levels::N900 => "#0f172a",
            Levels::N950 => "#020617",
        },
        Colors::Gray(number) => match number {
            Levels::N50 => "#f9fafb",
            Levels::N100 => "#f3f4f6",
            Levels::N200 => "#e5e7eb",
            Levels::N300 => "#d1d5db",
            Levels::N400 => "#9ca3af",
            Levels::N500 => "#6b7280",
            Levels::N600 => "#4b5563",
            Levels::N700 => "#374151",
            Levels::N800 => "#1f2937",
            Levels::N900 => "#111827",
            Levels::N950 => "#030712",
        },
        Colors::Zinc(number) => match number {
            Levels::N50 => "#fafafa",
            Levels::N100 => "#f4f4f5",
            Levels::N200 => "#e4e4e7",
            Levels::N300 => "#d4d4d8",
            Levels::N400 => "#a1a1aa",
            Levels::N500 => "#71717a",
            Levels::N600 => "#52525b",
            Levels::N700 => "#3f3f46",
            Levels::N800 => "#27272a",
            Levels::N900 => "#18181b",
            Levels::N950 => "#09090b",
        },
        Colors::Neutral(number) => match number {
            Levels::N50 => "#fafafa",
            Levels::N100 => "#f5f5f5",
            Levels::N200 => "#e5e5e5",
            Levels::N300 => "#d4d4d4",
            Levels::N400 => "#a3a3a3",
            Levels::N500 => "#737373",
            Levels::N600 => "#525252",
            Levels::N700 => "#404040",
            Levels::N800 => "#262626",
            Levels::N900 => "#171717",
            Levels::N950 => "#0a0a0a",
        },
        Colors::Stone(number) => match number {
            Levels::N50 => "#fafaf9",
            Levels::N100 => "#f5f5f4",
            Levels::N200 => "#e7e5e4",
            Levels::N300 => "#d6d3d1",
            Levels::N400 => "#a8a29e",
            Levels::N500 => "#78716c",
            Levels::N600 => "#57534e",
            Levels::N700 => "#44403c",
            Levels::N800 => "#292524",
            Levels::N900 => "#1c1917",
            Levels::N950 => "#0c0a09",
        },
        Colors::Red(number) => match number {
            Levels::N50 => "#fef2f2",
            Levels::N100 => "#fee2e2",
            Levels::N200 => "#fecaca",
            Levels::N300 => "#fca5a5",
            Levels::N400 => "#f87171",
            Levels::N500 => "#ef4444",
            Levels::N600 => "#dc2626",
            Levels::N700 => "#b91c1c",
            Levels::N800 => "#991b1b",
            Levels::N900 => "#7f1d1d",
            Levels::N950 => "#450a0a",
        },
        Colors::Orange(number) => match number {
            Levels::N50 => "#fff7ed",
            Levels::N100 => "#ffedd5",
            Levels::N200 => "#fed7aa",
            Levels::N300 => "#fdba74",
            Levels::N400 => "#fb923c",
            Levels::N500 => "#f97316",
            Levels::N600 => "#ea580c",
            Levels::N700 => "#c2410c",
            Levels::N800 => "#9a3412",
            Levels::N900 => "#7c2d12",
            Levels::N950 => "#431407",
        },
        Colors::Amber(number) => match number {
            Levels::N50 => "#fffbeb",
            Levels::N100 => "#fef3c7",
            Levels::N200 => "#fde68a",
            Levels::N300 => "#fcd34d",
            Levels::N400 => "#fbbf24",
            Levels::N500 => "#f59e0b",
            Levels::N600 => "#d97706",
            Levels::N700 => "#b45309",
            Levels::N800 => "#92400e",
            Levels::N900 => "#78350f",
            Levels::N950 => "#451a03",
        },
        Colors::Yellow(number) => match number {
            Levels::N50 => "#fefce8",
            Levels::N100 => "#fef9c3",
            Levels::N200 => "#fef08a",
            Levels::N300 => "#fde047",
            Levels::N400 => "#facc15",
            Levels::N500 => "#eab308",
            Levels::N600 => "#ca8a04",
            Levels::N700 => "#a16207",
            Levels::N800 => "#854d0e",
            Levels::N900 => "#713f12",
            Levels::N950 => "#422006",
        },
        Colors::Lime(number) => match number {
            Levels::N50 => "#f7fee7",
            Levels::N100 => "#ecfccb",
            Levels::N200 => "#d9f99d",
            Levels::N300 => "#bef264",
            Levels::N400 => "#a3e635",
            Levels::N500 => "#84cc16",
            Levels::N600 => "#65a30d",
            Levels::N700 => "#4d7c0f",
            Levels::N800 => "#3f6212",
            Levels::N900 => "#365314",
            Levels::N950 => "#1a2e05",
        },
        Colors::Green(number) => match number {
            Levels::N50 => "#f0fdf4",
            Levels::N100 => "#dcfce7",
            Levels::N200 => "#bbf7d0",
            Levels::N300 => "#86efac",
            Levels::N400 => "#4ade80",
            Levels::N500 => "#22c55e",
            Levels::N600 => "#16a34a",
            Levels::N700 => "#15803d",
            Levels::N800 => "#166534",
            Levels::N900 => "#14532d",
            Levels::N950 => "#052e16",
        },
        Colors::Emerald(number) => match number {
            Levels::N50 => "#ecfdf5",
            Levels::N100 => "#d1fae5",
            Levels::N200 => "#a7f3d0",
            Levels::N300 => "#6ee7b7",
            Levels::N400 => "#34d399",
            Levels::N500 => "#10b981",
            Levels::N600 => "#059669",
            Levels::N700 => "#047857",
            Levels::N800 => "#065f46",
            Levels::N900 => "#064e3b",
            Levels::N950 => "#022c22",
        },
        Colors::Teal(number) => match number {
            Levels::N50 => "#f0fdfa",
            Levels::N100 => "#ccfbf1",
            Levels::N200 => "#99f6e4",
            Levels::N300 => "#5eead4",
            Levels::N400 => "#2dd4bf",
            Levels::N500 => "#14b8a6",
            Levels::N600 => "#0d9488",
            Levels::N700 => "#0f766e",
            Levels::N800 => "#115e59",
            Levels::N900 => "#134e4a",
            Levels::N950 => "#042f2e",
        },
        Colors::Cyan(number) => match number {
            Levels::N50 => "#ecfeff",
            Levels::N100 => "#cffafe",
            Levels::N200 => "#a5f3fc",
            Levels::N300 => "#67e8f9",
            Levels::N400 => "#22d3ee",
            Levels::N500 => "#06b6d4",
            Levels::N600 => "#0891b2",
            Levels::N700 => "#0e7490",
            Levels::N800 => "#155e75",
            Levels::N900 => "#164e63",
            Levels::N950 => "#083344",
        },
        Colors::Sky(number) => match number {
            Levels::N50 => "#f0f9ff",
            Levels::N100 => "#e0f2fe",
            Levels::N200 => "#bae6fd",
            Levels::N300 => "#7dd3fc",
            Levels::N400 => "#38bdf8",
            Levels::N500 => "#0ea5e9",
            Levels::N600 => "#0284c7",
            Levels::N700 => "#0369a1",
            Levels::N800 => "#075985",
            Levels::N900 => "#0c4a6e",
            Levels::N950 => "#082f49",
        },
        Colors::Blue(number) => match number {
            Levels::N50 => "#eff6ff",
            Levels::N100 => "#dbeafe",
            Levels::N200 => "#bfdbfe",
            Levels::N300 => "#93c5fd",
            Levels::N400 => "#60a5fa",
            Levels::N500 => "#3b82f6",
            Levels::N600 => "#2563eb",
            Levels::N700 => "#1d4ed8",
            Levels::N800 => "#1e40af",
            Levels::N900 => "#1e3a8a",
            Levels::N950 => "#172554",
        },
        Colors::Indigo(number) => match number {
            Levels::N50 => "#eef2ff",
            Levels::N100 => "#e0e7ff",
            Levels::N200 => "#c7d2fe",
            Levels::N300 => "#a5b4fc",
            Levels::N400 => "#818cf8",
            Levels::N500 => "#6366f1",
            Levels::N600 => "#4f46e5",
            Levels::N700 => "#4338ca",
            Levels::N800 => "#3730a3",
            Levels::N900 => "#312e81",
            Levels::N950 => "#1e1b4b",
        },
        Colors::Violet(number) => match number {
            Levels::N50 => "#f5f3ff",
            Levels::N100 => "#ede9fe",
            Levels::N200 => "#ddd6fe",
            Levels::N300 => "#c4b5fd",
            Levels::N400 => "#a78bfa",
            Levels::N500 => "#8b5cf6",
            Levels::N600 => "#7c3aed",
            Levels::N700 => "#6d28d9",
            Levels::N800 => "#5b21b6",
            Levels::N900 => "#4c1d95",
            Levels::N950 => "#2e1065",
        },
        Colors::Purple(number) => match number {
            Levels::N50 => "#faf5ff",
            Levels::N100 => "#f3e8ff",
            Levels::N200 => "#e9d5ff",
            Levels::N300 => "#d8b4fe",
            Levels::N400 => "#c084fc",
            Levels::N500 => "#a855f7",
            Levels::N600 => "#9333ea",
            Levels::N700 => "#7e22ce",
            Levels::N800 => "#6b21a8",
            Levels::N900 => "#581c87",
            Levels::N950 => "#3b0764",
        },
        Colors::Fuchsia(number) => match number {
            Levels::N50 => "#fdf4ff",
            Levels::N100 => "#fae8ff",
            Levels::N200 => "#f5d0fe",
            Levels::N300 => "#f0abfc",
            Levels::N400 => "#e879f9",
            Levels::N500 => "#d946ef",
            Levels::N600 => "#c026d3",
            Levels::N700 => "#a21caf",
            Levels::N800 => "#86198f",
            Levels::N900 => "#701a75",
            Levels::N950 => "#4a044e",
        },
        Colors::Pink(number) => match number {
            Levels::N50 => "#fdf2f8",
            Levels::N100 => "#fce7f3",
            Levels::N200 => "#fbcfe8",
            Levels::N300 => "#f9a8d4",
            Levels::N400 => "#f472b6",
            Levels::N500 => "#ec4899",
            Levels::N600 => "#db2777",
            Levels::N700 => "#be185d",
            Levels::N800 => "#9d174d",
            Levels::N900 => "#831843",
            Levels::N950 => "#500724",
        },
        Colors::Rose(number) => match number {
            Levels::N50 => "#fff1f2",
            Levels::N100 => "#ffe4e6",
            Levels::N200 => "#fecdd3",
            Levels::N300 => "#fda4af",
            Levels::N400 => "#fb7185",
            Levels::N500 => "#f43f5e",
            Levels::N600 => "#e11d48",
            Levels::N700 => "#be123c",
            Levels::N800 => "#9f1239",
            Levels::N900 => "#881337",
            Levels::N950 => "#4c0519",
        },
        Colors::Custom(color) => &color.to_owned(),
    };
    return color.to_string();
}
