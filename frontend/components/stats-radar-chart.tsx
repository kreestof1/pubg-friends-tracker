'use client';

import { Radar, RadarChart, PolarGrid, PolarAngleAxis, PolarRadiusAxis, ResponsiveContainer, Tooltip, Legend } from 'recharts';
import { PlayerStats } from '@/lib/types';

interface PlayerData {
  player_id: string;
  name: string;
  stats: PlayerStats;
}

interface StatsRadarChartProps {
  data: PlayerData[];
}

const COLORS = [
  '#3b82f6', // blue
  '#ef4444', // red
  '#10b981', // green
  '#f59e0b', // orange
  '#8b5cf6', // purple
  '#ec4899', // pink
  '#14b8a6', // teal
  '#f97316', // orange
  '#06b6d4', // cyan
  '#a855f7', // violet
];

export function StatsRadarChart({ data }: StatsRadarChartProps) {
  // Normalize stats to 0-100 scale for radar chart
  const normalizeValue = (value: number, max: number) => {
    return Math.min((value / max) * 100, 100);
  };

  // Calculate max values for normalization
  const maxValues = {
    kills: Math.max(...data.map(p => p.stats.kills), 5),
    damage_dealt: Math.max(...data.map(p => p.stats.damage_dealt), 500),
    kd_ratio: Math.max(...data.map(p => p.stats.kd_ratio), 3),
    win_rate: 100, // Already in percentage
    survival_time: Math.max(...data.map(p => p.stats.survival_time), 1500),
  };

  const chartData = [
    {
      metric: 'Kills',
      ...Object.fromEntries(
        data.map((player, idx) => [
          `player${idx}`,
          normalizeValue(player.stats.kills, maxValues.kills),
        ])
      ),
    },
    {
      metric: 'K/D',
      ...Object.fromEntries(
        data.map((player, idx) => [
          `player${idx}`,
          normalizeValue(player.stats.kd_ratio, maxValues.kd_ratio),
        ])
      ),
    },
    {
      metric: 'Win Rate',
      ...Object.fromEntries(
        data.map((player, idx) => [
          `player${idx}`,
          player.stats.win_rate * 100,
        ])
      ),
    },
    {
      metric: 'Damage',
      ...Object.fromEntries(
        data.map((player, idx) => [
          `player${idx}`,
          normalizeValue(player.stats.damage_dealt, maxValues.damage_dealt),
        ])
      ),
    },
    {
      metric: 'Survival',
      ...Object.fromEntries(
        data.map((player, idx) => [
          `player${idx}`,
          normalizeValue(player.stats.survival_time, maxValues.survival_time),
        ])
      ),
    },
  ];

  return (
    <div className="w-full h-full">
      <h3 className="text-lg font-semibold mb-4">Multi-Metric Comparison</h3>
      <ResponsiveContainer width="100%" height="100%">
        <RadarChart data={chartData}>
          <PolarGrid className="stroke-gray-200 dark:stroke-gray-700" />
          <PolarAngleAxis 
            dataKey="metric" 
            className="text-sm fill-gray-600 dark:fill-gray-400"
            tick={{ fontSize: 12 }}
          />
          <PolarRadiusAxis 
            angle={90} 
            domain={[0, 100]}
            className="text-xs fill-gray-500 dark:fill-gray-500"
            tick={{ fontSize: 10 }}
          />
          <Tooltip
            contentStyle={{
              backgroundColor: 'rgba(255, 255, 255, 0.95)',
              border: '1px solid #e5e7eb',
              borderRadius: '8px',
            }}
            formatter={(value: number | undefined) => value !== undefined ? value.toFixed(1) : ''}
          />
          <Legend />
          {data.map((player, idx) => (
            <Radar
              key={player.player_id}
              name={player.name}
              dataKey={`player${idx}`}
              stroke={COLORS[idx % COLORS.length]}
              fill={COLORS[idx % COLORS.length]}
              fillOpacity={0.2}
            />
          ))}
        </RadarChart>
      </ResponsiveContainer>
    </div>
  );
}
