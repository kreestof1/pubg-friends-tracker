'use client';

import { useState } from 'react';
import Link from 'next/link';
import { Users, TrendingUp, Target, Zap } from 'lucide-react';
import { useDashboard, usePlayers } from '@/hooks/use-api';
import { LoadingSpinner } from '@/components/loading-spinner';
import { ErrorAlert } from '@/components/error-alert';
import { EmptyState } from '@/components/empty-state';
import { MetricCard } from '@/components/metric-card';
import { ComparisonBarChart } from '@/components/comparison-bar-chart';
import { StatsRadarChart } from '@/components/stats-radar-chart';
import { StatsLeaderboard } from '@/components/stats-leaderboard';
import type { Period, GameMode, Shard } from '@/lib/types';

export default function DashboardPage() {
  const [selectedPlayers, setSelectedPlayers] = useState<string[]>([]);
  const [period, setPeriod] = useState<Period>('7d');
  const [mode, setMode] = useState<GameMode>('solo');
  const [shard, setShard] = useState<Shard>('steam');

  const { players, isLoading: playersLoading } = usePlayers();
  const { dashboard, isLoading, isError, mutate } = useDashboard(
    selectedPlayers,
    period,
    mode,
    shard
  );

  const handlePlayerToggle = (playerId: string) => {
    setSelectedPlayers((prev) =>
      prev.includes(playerId)
        ? prev.filter((id) => id !== playerId)
        : prev.length < 10
        ? [...prev, playerId]
        : prev
    );
  };

  if (playersLoading) {
    return (
      <div className="flex h-screen items-center justify-center">
        <LoadingSpinner size="lg" />
      </div>
    );
  }

  return (
    <div className="container mx-auto p-6 space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-4xl font-bold">PUBG Friends Tracker</h1>
        <p className="mt-2 text-muted-foreground">
          Compare your friends stats and track their performance
        </p>
      </div>

      {/* Player Selection */}
      <div className="rounded-lg border bg-card p-6">
        <h2 className="text-xl font-semibold mb-4">Select Players</h2>
        <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-3">
          {players?.map((player) => (
            <button
              key={player.id}
              onClick={() => handlePlayerToggle(player.id)}
              className={`rounded-md border p-3 text-sm font-medium transition-all ${
                selectedPlayers.includes(player.id)
                  ? 'border-primary bg-primary text-primary-foreground'
                  : 'border-border hover:border-primary hover:bg-muted'
              }`}
            >
              {player.name}
            </button>
          ))}
        </div>
        {selectedPlayers.length >= 10 && (
          <p className="mt-3 text-sm text-yellow-600">
            Maximum 10 players can be selected
          </p>
        )}
      </div>

      {/* Filters */}
      <div className="flex flex-wrap gap-4">
        {/* Period Filter */}
        <div>
          <label className="block text-sm font-medium mb-2">Period</label>
          <div className="flex gap-2">
            {(['7d', '30d', '90d'] as Period[]).map((p) => (
              <button
                key={p}
                onClick={() => setPeriod(p)}
                className={`rounded-md px-4 py-2 text-sm font-medium transition-colors ${
                  period === p
                    ? 'bg-primary text-primary-foreground'
                    : 'bg-muted hover:bg-muted/80'
                }`}
              >
                {p === '7d' ? '7 Days' : p === '30d' ? '30 Days' : '90 Days'}
              </button>
            ))}
          </div>
        </div>

        {/* Mode Filter */}
        <div>
          <label className="block text-sm font-medium mb-2">Game Mode</label>
          <div className="flex gap-2">
            {(['solo', 'duo', 'squad'] as GameMode[]).map((m) => (
              <button
                key={m}
                onClick={() => setMode(m)}
                className={`rounded-md px-4 py-2 text-sm font-medium capitalize transition-colors ${
                  mode === m
                    ? 'bg-primary text-primary-foreground'
                    : 'bg-muted hover:bg-muted/80'
                }`}
              >
                {m}
              </button>
            ))}
          </div>
        </div>

        {/* Shard Filter */}
        <div>
          <label className="block text-sm font-medium mb-2">Platform</label>
          <select
            value={shard}
            onChange={(e) => setShard(e.target.value as Shard)}
            className="rounded-md border bg-background px-4 py-2 text-sm font-medium"
          >
            <option value="steam">Steam</option>
            <option value="xbox">Xbox</option>
            <option value="psn">PlayStation</option>
            <option value="kakao">Kakao</option>
          </select>
        </div>
      </div>

      {/* Dashboard Content */}
      {selectedPlayers.length === 0 ? (
        <EmptyState
          icon={Users}
          title="No players selected"
          description="Select at least one player to view stats comparison"
        />
      ) : isLoading ? (
        <div className="flex items-center justify-center py-12">
          <LoadingSpinner size="lg" />
        </div>
      ) : isError ? (
        <ErrorAlert
          message="Failed to load dashboard stats"
          onRetry={() => mutate()}
        />
      ) : dashboard ? (
        <div className="space-y-6">
          {/* Summary Cards */}
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            {dashboard.players.map((player) => (
              <div key={player.player_id} className="space-y-2">
                <Link 
                  href={`/players/${player.player_id}`}
                  className="font-semibold hover:text-primary transition-colors"
                >
                  {player.name}
                </Link>
                <div className="grid grid-cols-2 gap-2">
                  <MetricCard
                    icon={Target}
                    label="K/D Ratio"
                    value={player.stats.kd_ratio.toFixed(2)}
                    variant="primary"
                  />
                  <MetricCard
                    icon={TrendingUp}
                    label="Win Rate"
                    value={`${player.stats.win_rate.toFixed(1)}%`}
                    variant="success"
                  />
                  <MetricCard
                    icon={Zap}
                    label="Kills"
                    value={player.stats.kills}
                    variant="warning"
                  />
                  <MetricCard
                    icon={Users}
                    label="Matches"
                    value={player.stats.matches_played}
                    variant="default"
                  />
                </div>
              </div>
            ))}
          </div>

          {/* Leaderboard */}
          <div className="rounded-lg border bg-card p-6">
            <StatsLeaderboard data={dashboard.players} />
          </div>

          {/* Bar Charts */}
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <div className="rounded-lg border bg-card p-6 h-80">
              <ComparisonBarChart
                data={dashboard.players}
                metric="kd_ratio"
                title="K/D Ratio Comparison"
              />
            </div>
            <div className="rounded-lg border bg-card p-6 h-80">
              <ComparisonBarChart
                data={dashboard.players}
                metric="win_rate"
                title="Win Rate Comparison"
              />
            </div>
          </div>

          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <div className="rounded-lg border bg-card p-6 h-80">
              <ComparisonBarChart
                data={dashboard.players}
                metric="kills"
                title="Average Kills Comparison"
              />
            </div>
            <div className="rounded-lg border bg-card p-6 h-80">
              <ComparisonBarChart
                data={dashboard.players}
                metric="damage_dealt"
                title="Average Damage Comparison"
              />
            </div>
          </div>

          {/* Radar Chart */}
          <div className="rounded-lg border bg-card p-6 h-96">
            <StatsRadarChart data={dashboard.players} />
          </div>
        </div>
      ) : null}
    </div>
  );
}
