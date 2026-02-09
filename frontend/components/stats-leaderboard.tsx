'use client';

import { useState } from 'react';
import { PlayerStats } from '@/lib/types';
import { ArrowUpDown, ArrowUp, ArrowDown, Trophy } from 'lucide-react';
import { cn } from '@/lib/utils';

interface PlayerData {
  player_id: string;
  name: string;
  stats: PlayerStats;
}

interface StatsLeaderboardProps {
  data: PlayerData[];
}

type SortKey = 'kills' | 'damage_dealt' | 'kd_ratio' | 'win_rate' | 'matches_played';
type SortOrder = 'asc' | 'desc';

export function StatsLeaderboard({ data }: StatsLeaderboardProps) {
  const [sortKey, setSortKey] = useState<SortKey>('kd_ratio');
  const [sortOrder, setSortOrder] = useState<SortOrder>('desc');

  const handleSort = (key: SortKey) => {
    if (sortKey === key) {
      setSortOrder(sortOrder === 'asc' ? 'desc' : 'asc');
    } else {
      setSortKey(key);
      setSortOrder('desc');
    }
  };

  const sortedData = [...data].sort((a, b) => {
    const aValue = a.stats[sortKey];
    const bValue = b.stats[sortKey];
    return sortOrder === 'asc' ? aValue - bValue : bValue - aValue;
  });

  const SortIcon = ({ columnKey }: { columnKey: SortKey }) => {
    if (sortKey !== columnKey) {
      return <ArrowUpDown className="w-4 h-4" />;
    }
    return sortOrder === 'asc' ? (
      <ArrowUp className="w-4 h-4" />
    ) : (
      <ArrowDown className="w-4 h-4" />
    );
  };

  return (
    <div className="w-full">
      <h3 className="text-lg font-semibold mb-4 flex items-center gap-2">
        <Trophy className="w-5 h-5 text-yellow-500" />
        Leaderboard
      </h3>
      <div className="overflow-x-auto">
        <table className="w-full border-collapse">
          <thead>
            <tr className="border-b border-gray-200 dark:border-gray-700">
              <th className="text-left p-3 font-semibold text-sm">Rank</th>
              <th className="text-left p-3 font-semibold text-sm">Player</th>
              <th 
                className="text-right p-3 font-semibold text-sm cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
                onClick={() => handleSort('kills')}
              >
                <div className="flex items-center justify-end gap-1">
                  Kills
                  <SortIcon columnKey="kills" />
                </div>
              </th>
              <th 
                className="text-right p-3 font-semibold text-sm cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
                onClick={() => handleSort('damage_dealt')}
              >
                <div className="flex items-center justify-end gap-1">
                  Damage
                  <SortIcon columnKey="damage_dealt" />
                </div>
              </th>
              <th 
                className="text-right p-3 font-semibold text-sm cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
                onClick={() => handleSort('kd_ratio')}
              >
                <div className="flex items-center justify-end gap-1">
                  K/D
                  <SortIcon columnKey="kd_ratio" />
                </div>
              </th>
              <th 
                className="text-right p-3 font-semibold text-sm cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
                onClick={() => handleSort('win_rate')}
              >
                <div className="flex items-center justify-end gap-1">
                  Win Rate
                  <SortIcon columnKey="win_rate" />
                </div>
              </th>
              <th 
                className="text-right p-3 font-semibold text-sm cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
                onClick={() => handleSort('matches_played')}
              >
                <div className="flex items-center justify-end gap-1">
                  Matches
                  <SortIcon columnKey="matches_played" />
                </div>
              </th>
            </tr>
          </thead>
          <tbody>
            {sortedData.map((player, index) => (
              <tr
                key={player.player_id}
                className={cn(
                  'border-b border-gray-100 dark:border-gray-800 hover:bg-gray-50 dark:hover:bg-gray-800/50 transition-colors',
                  index === 0 && 'bg-yellow-50 dark:bg-yellow-900/10'
                )}
              >
                <td className="p-3">
                  <div className="flex items-center gap-2">
                    {index === 0 && (
                      <Trophy className="w-4 h-4 text-yellow-500" />
                    )}
                    <span className={cn(
                      'font-semibold',
                      index === 0 && 'text-yellow-600 dark:text-yellow-500',
                      index === 1 && 'text-gray-500',
                      index === 2 && 'text-orange-600'
                    )}>
                      #{index + 1}
                    </span>
                  </div>
                </td>
                <td className="p-3 font-medium">{player.name}</td>
                <td className="p-3 text-right font-mono text-sm">
                  {player.stats.kills.toFixed(2)}
                </td>
                <td className="p-3 text-right font-mono text-sm">
                  {player.stats.damage_dealt.toFixed(0)}
                </td>
                <td className="p-3 text-right font-mono text-sm">
                  {player.stats.kd_ratio.toFixed(2)}
                </td>
                <td className="p-3 text-right font-mono text-sm">
                  {(player.stats.win_rate * 100).toFixed(1)}%
                </td>
                <td className="p-3 text-right font-mono text-sm">
                  {player.stats.matches_played}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
