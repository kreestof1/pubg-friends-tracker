// TypeScript types and interfaces

export interface Player {
  id: string;
  account_id: string;
  name: string;
  shard: string;
  last_matches?: string[];
  last_refreshed_at?: Date;
  created_at: Date;
  summary?: Record<string, unknown>;
}

export interface PlayerStats {
  player_id: string;
  period: string;
  mode: string;
  shard: string;
  kills: number;
  deaths: number;
  kd_ratio: number;
  win_rate: number;
  damage_dealt: number;
  survival_time: number;
  top1_count: number;
  matches_played: number;
  computed_at: Date;
}

export interface DashboardData {
  players: Array<{
    player_id: string;
    name: string;
    stats: PlayerStats;
  }>;
  period: string;
  mode: string;
}

export interface Match {
  id: string;
  shard: string;
  match_url: string;
  telemetry_url?: string;
}

export type Shard = 'steam' | 'xbox' | 'psn' | 'kakao' | 'stadia';
export type GameMode = 'solo' | 'duo' | 'squad';
export type Period = 'last7d' | 'last30d' | 'last90d';
