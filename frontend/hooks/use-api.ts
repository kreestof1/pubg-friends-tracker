// Custom React hooks for data fetching with SWR
'use client';

import useSWR from 'swr';
import type { Player, PlayerStats, DashboardData } from '@/lib/types';
import {
  getPlayers,
  getPlayer,
  getDashboardStats,
  getPlayerMatches,
  getPlayerStats,
} from '@/lib/api';

// Fetcher function for SWR
const fetcher = async (url: string) => {
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error('Failed to fetch data');
  }
  return response.json();
};

/**
 * Hook to fetch and manage list of players
 */
export function usePlayers(page: number = 1, limit: number = 10) {
  const { data, error, isLoading, mutate } = useSWR<Player[]>(
    `/players?page=${page}&limit=${limit}`,
    () => getPlayers(page, limit),
    {
      revalidateOnFocus: false,
      dedupingInterval: 5000,
    }
  );

  return {
    players: data,
    isLoading,
    isError: error,
    mutate,
  };
}

/**
 * Hook to fetch a single player by ID
 */
export function usePlayer(id: string | null) {
  const { data, error, isLoading, mutate } = useSWR<Player>(
    id ? `/players/${id}` : null,
    id ? () => getPlayer(id) : null,
    {
      revalidateOnFocus: false,
    }
  );

  return {
    player: data,
    isLoading,
    isError: error,
    mutate,
  };
}

/**
 * Hook to fetch player matches
 */
export function usePlayerMatches(playerId: string | null) {
  const { data, error, isLoading, mutate } = useSWR<string[]>(
    playerId ? `/players/${playerId}/matches` : null,
    playerId ? () => getPlayerMatches(playerId) : null,
    {
      revalidateOnFocus: false,
    }
  );

  return {
    matches: data,
    isLoading,
    isError: error,
    mutate,
  };
}

/**
 * Hook to fetch dashboard stats for multiple players
 */
export function useDashboard(
  playerIds: string[],
  period: string = 'last7d',
  mode: string = 'solo',
  shard: string = 'steam'
) {
  const key =
    playerIds.length > 0
      ? `/dashboard?ids=${playerIds.join(',')}&period=${period}&mode=${mode}&shard=${shard}`
      : null;

  const { data, error, isLoading, mutate } = useSWR<DashboardData>(
    key,
    () => getDashboardStats(playerIds, period, mode, shard),
    {
      revalidateOnFocus: false,
      dedupingInterval: 10000, // Cache for 10 seconds
      refreshInterval: 0, // No auto-refresh by default
    }
  );

  return {
    dashboard: data,
    isLoading,
    isError: error,
    mutate,
  };
}

/**
 * Hook to fetch player stats with filters
 */
export function usePlayerStats(
  playerId: string | null,
  period: string = 'last7d',
  mode: string = 'solo',
  shard: string = 'steam'
) {
  const key = playerId
    ? `/players/${playerId}/stats?period=${period}&mode=${mode}&shard=${shard}`
    : null;

  const { data, error, isLoading, mutate } = useSWR<PlayerStats>(
    key,
    playerId ? () => getPlayerStats(playerId, period, mode, shard) : null,
    {
      revalidateOnFocus: false,
      dedupingInterval: 10000,
    }
  );

  return {
    stats: data,
    isLoading,
    isError: error,
    mutate,
  };
}
