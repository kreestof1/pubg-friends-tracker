'use client';

import { BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts';
import { PlayerStats } from '@/lib/types';

interface PlayerData {
  player_id: string;
  name: string;
  stats: PlayerStats;
}

interface ComparisonBarChartProps {
  data: PlayerData[];
  metric: 'kills' | 'damage_dealt' | 'kd_ratio' | 'win_rate';
  title: string;
}

const metricConfig = {
  kills: {
    label: 'Average Kills',
    color: '#3b82f6',
    format: (value: number) => value.toFixed(2),
  },
  damage_dealt: {
    label: 'Average Damage',
    color: '#ef4444',
    format: (value: number) => value.toFixed(0),
  },
  kd_ratio: {
    label: 'K/D Ratio',
    color: '#10b981',
    format: (value: number) => value.toFixed(2),
  },
  win_rate: {
    label: 'Win Rate (%)',
    color: '#f59e0b',
    format: (value: number) => value.toFixed(1),
  },
};

export function ComparisonBarChart({ data, metric, title }: ComparisonBarChartProps) {
  const config = metricConfig[metric];

  const chartData = data.map((player) => ({
    name: player.name,
    value: player.stats[metric],
  }));

  return (
    <div className="w-full h-full">
      <h3 className="text-lg font-semibold mb-4">{title}</h3>
      <ResponsiveContainer width="100%" height="100%">
        <BarChart data={chartData} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
          <CartesianGrid strokeDasharray="3 3" className="stroke-gray-200 dark:stroke-gray-700" />
          <XAxis 
            dataKey="name" 
            className="text-sm fill-gray-600 dark:fill-gray-400"
            tick={{ fontSize: 12 }}
          />
          <YAxis 
            className="text-sm fill-gray-600 dark:fill-gray-400"
            tick={{ fontSize: 12 }}
          />
          <Tooltip
            contentStyle={{
              backgroundColor: 'rgba(255, 255, 255, 0.95)',
              border: '1px solid #e5e7eb',
              borderRadius: '8px',
            }}
            formatter={(value: number | undefined) => value !== undefined ? [config.format(value), config.label] : ['', '']}
          />
          <Legend />
          <Bar 
            dataKey="value" 
            fill={config.color} 
            name={config.label}
            radius={[8, 8, 0, 0]}
          />
        </BarChart>
      </ResponsiveContainer>
    </div>
  );
}
