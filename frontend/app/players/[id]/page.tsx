'use client';

import { use } from 'react';
import { useState } from 'react';
import Link from 'next/link';
import { usePlayer, usePlayerStats, usePlayerMatches } from '@/hooks/use-api';
import { LoadingSpinner } from '@/components/loading-spinner';
import { ErrorAlert } from '@/components/error-alert';
import { MetricCard } from '@/components/metric-card';
import { ArrowLeft, RefreshCw, Users, Target, TrendingUp, Zap, Clock } from 'lucide-react';
import type { Period, GameMode, Shard } from '@/lib/types';

interface PageProps {
  params: Promise<{
    id: string;
  }>;
}

export default function PlayerDetailsPage({ params }: PageProps) {
  const { id } = use(params);
  const [period, setPeriod] = useState<Period>('last7d');
  const [mode, setMode] = useState<GameMode>('solo');
  const [shard, setShard] = useState<Shard>('steam');

  const { player, isLoading: playerLoading, isError: playerError, mutate: mutatePlayer } = usePlayer(id);
  const { stats, isLoading: statsLoading, isError: statsError, mutate: mutateStats } = usePlayerStats(id, period, mode, shard);
  const { matches, isLoading: matchesLoading, isError: matchesError } = usePlayerMatches(id);

  const handleRefresh = async () => {
    await Promise.all([mutatePlayer(), mutateStats()]);
  };

  if (playerLoading || statsLoading) {
    return (
      <div className="flex h-screen items-center justify-center">
        <LoadingSpinner size="lg" />
      </div>
    );
  }

  if (playerError || !player) {
    return (
      <div className="container mx-auto p-6">
        <ErrorAlert
          message="Failed to load player details"
          onRetry={() => mutatePlayer()}
        />
      </div>
    );
  }

  return (
    <div className="container mx-auto p-6 space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <Link
            href="/players"
            className="rounded-md p-2 hover:bg-muted transition-colors"
          >
            <ArrowLeft className="w-5 h-5" />
          </Link>
          <div>
            <h1 className="text-4xl font-bold">{player.name}</h1>
            <p className="mt-1 text-muted-foreground">
              Platform: {player.shard} • Last updated:{' '}
              {player.last_refreshed_at
                ? new Date(player.last_refreshed_at).toLocaleDateString()
                : 'Never'}
            </p>
          </div>
        </div>
        <button
          onClick={handleRefresh}
          disabled={statsLoading}
          className="inline-flex items-center gap-2 rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50 transition-colors"
        >
          <RefreshCw className={`w-4 h-4 ${statsLoading ? 'animate-spin' : ''}`} />
          Refresh Stats
        </button>
      </div>

      {/* Filters */}
      <div className="flex flex-wrap gap-4">
        {/* Period Filter */}
        <div>
          <label className="block text-sm font-medium mb-2">Period</label>
          <div className="flex gap-2">
            {(['last7d', 'last30d', 'last90d'] as Period[]).map((p) => (
              <button
                key={p}
                onClick={() => setPeriod(p)}
                className={`rounded-md px-4 py-2 text-sm font-medium transition-colors ${
                  period === p
                    ? 'bg-primary text-primary-foreground'
                    : 'bg-muted hover:bg-muted/80'
                }`}
              >
                {p === 'last7d' ? '7 Days' : p === 'last30d' ? '30 Days' : '90 Days'}
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

      {/* Stats Content */}
      {statsError ? (
        <ErrorAlert
          message="Failed to load player statistics"
          onRetry={() => mutateStats()}
        />
      ) : stats ? (
        <div className="space-y-6">
          {/* Main Stats Grid */}
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            <MetricCard
              icon={Target}
              label="K/D Ratio"
              value={stats.kd_ratio.toFixed(2)}
              variant="primary"
            />
            <MetricCard
              icon={TrendingUp}
              label="Win Rate"
              value={`${(stats.win_rate * 100).toFixed(1)}%`}
              variant="success"
            />
            <MetricCard
              icon={Zap}
              label="Avg Kills"
              value={stats.kills.toFixed(2)}
              variant="warning"
            />
            <MetricCard
              icon={Users}
              label="Matches"
              value={stats.matches_played}
              variant="default"
            />
          </div>

          {/* Secondary Stats */}
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div className="rounded-lg border bg-card p-6">
              <div className="flex items-center gap-2 text-sm text-muted-foreground mb-2">
                <Target className="w-4 h-4" />
                <span>Combat Stats</span>
              </div>
              <div className="space-y-2">
                <div className="flex justify-between">
                  <span className="text-sm">Total Kills</span>
                  <span className="font-semibold">{(stats.kills * stats.matches_played).toFixed(0)}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-sm">Total Deaths</span>
                  <span className="font-semibold">{(stats.deaths * stats.matches_played).toFixed(0)}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-sm">Avg Damage</span>
                  <span className="font-semibold">{stats.damage_dealt.toFixed(0)}</span>
                </div>
              </div>
            </div>

            <div className="rounded-lg border bg-card p-6">
              <div className="flex items-center gap-2 text-sm text-muted-foreground mb-2">
                <TrendingUp className="w-4 h-4" />
                <span>Performance</span>
              </div>
              <div className="space-y-2">
                <div className="flex justify-between">
                  <span className="text-sm">Wins</span>
                  <span className="font-semibold">{stats.top1_count}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-sm">Win Rate</span>
                  <span className="font-semibold">{(stats.win_rate * 100).toFixed(1)}%</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-sm">Matches</span>
                  <span className="font-semibold">{stats.matches_played}</span>
                </div>
              </div>
            </div>

            <div className="rounded-lg border bg-card p-6">
              <div className="flex items-center gap-2 text-sm text-muted-foreground mb-2">
                <Clock className="w-4 h-4" />
                <span>Survival</span>
              </div>
              <div className="space-y-2">
                <div className="flex justify-between">
                  <span className="text-sm">Avg Time</span>
                  <span className="font-semibold">{(stats.survival_time / 60).toFixed(1)} min</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-sm">Total Time</span>
                  <span className="font-semibold">{((stats.survival_time * stats.matches_played) / 3600).toFixed(1)} hrs</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      ) : null}

      {/* Recent Matches */}
      <div className="rounded-lg border bg-card p-6">
        <h3 className="text-xl font-semibold mb-4">Recent Matches</h3>
        {matchesLoading ? (
          <div className="flex items-center justify-center py-8">
            <LoadingSpinner />
          </div>
        ) : matchesError ? (
          <ErrorAlert message="Failed to load match history" />
        ) : matches && matches.length > 0 ? (
          <div className="space-y-2">
            {matches.slice(0, 10).map((matchId) => (
              <div
                key={matchId}
                className="flex items-center justify-between p-3 rounded-md border hover:bg-muted transition-colors"
              >
                <div>
                  <span className="font-medium font-mono text-sm">{matchId}</span>
                  <p className="text-xs text-muted-foreground mt-1">{shard}</p>
                </div>
              </div>
            ))}
          </div>
        ) : (
          <p className="text-center text-muted-foreground py-8">No matches found</p>
        )}
      </div>
    </div>
  );
}
