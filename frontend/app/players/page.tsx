'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';
import { usePlayers } from '@/hooks/use-api';
import { refreshPlayer } from '@/lib/api';
import { LoadingSpinner } from '@/components/loading-spinner';
import { ErrorAlert } from '@/components/error-alert';
import { EmptyState } from '@/components/empty-state';
import { PlayerCard } from '@/components/player-card';
import { Search, Users, Plus } from 'lucide-react';
import type { Shard } from '@/lib/types';

export default function PlayersPage() {
  const router = useRouter();
  const [page, setPage] = useState(1);
  const [searchQuery, setSearchQuery] = useState('');
  const [shardFilter, setShardFilter] = useState<Shard | 'all'>('all');
  const [refreshingId, setRefreshingId] = useState<string | null>(null);
  const limit = 10;

  const { players, isLoading, isError, mutate } = usePlayers(page, limit);

  // Filter players by search query and shard
  const filteredPlayers = players?.filter((player) => {
    const matchesSearch = player.name.toLowerCase().includes(searchQuery.toLowerCase());
    const matchesShard = shardFilter === 'all' || player.shard === shardFilter;
    return matchesSearch && matchesShard;
  });

  const handleRefresh = async (playerId: string) => {
    try {
      setRefreshingId(playerId);
      await refreshPlayer(playerId);
      // Revalidate the players list after successful refresh
      await mutate();
    } catch (error) {
      console.error('Failed to refresh player:', error);
      // TODO: Show error toast/notification
    } finally {
      setRefreshingId(null);
    }
  };

  const totalPages = players ? Math.ceil(players.length / limit) : 1;

  return (
    <div className="container mx-auto p-6 space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-4xl font-bold">Players</h1>
          <p className="mt-2 text-muted-foreground">
            Manage and track all your PUBG friends
          </p>
        </div>
        <Link
          href="/players/new"
          className="inline-flex items-center gap-2 rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 transition-colors"
        >
          <Plus className="w-4 h-4" />
          Add Player
        </Link>
      </div>

      {/* Search and Filters */}
      <div className="flex flex-col sm:flex-row gap-4">
        {/* Search Input */}
        <div className="flex-1 relative">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-muted-foreground" />
          <input
            type="text"
            placeholder="Search players by name..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full pl-10 pr-4 py-2 rounded-md border bg-background focus:outline-none focus:ring-2 focus:ring-primary"
          />
        </div>

        {/* Shard Filter */}
        <select
          value={shardFilter}
          onChange={(e) => setShardFilter(e.target.value as Shard | 'all')}
          className="rounded-md border bg-background px-4 py-2 text-sm font-medium focus:outline-none focus:ring-2 focus:ring-primary"
        >
          <option value="all">All Platforms</option>
          <option value="steam">Steam</option>
          <option value="xbox">Xbox</option>
          <option value="psn">PlayStation</option>
          <option value="kakao">Kakao</option>
        </select>
      </div>

      {/* Players List */}
      {isLoading ? (
        <div className="flex items-center justify-center py-12">
          <LoadingSpinner size="lg" />
        </div>
      ) : isError ? (
        <ErrorAlert
          message="Failed to load players"
          onRetry={() => mutate()}
        />
      ) : !filteredPlayers || filteredPlayers.length === 0 ? (
        <EmptyState
          icon={Users}
          title="No players found"
          description={
            searchQuery || shardFilter !== 'all'
              ? 'Try adjusting your search or filters'
              : 'Add your first player to get started'
          }
          action={
            !searchQuery && shardFilter === 'all'
              ? {
                  label: 'Add Player',
                  onClick: () => router.push('/players/new'),
                }
              : undefined
          }
        />
      ) : (
        <>
          {/* Players Grid */}
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {filteredPlayers.map((player) => (
              <PlayerCard
                key={player.id}
                player={player}
                onRefresh={() => handleRefresh(player.id)}
                isRefreshing={refreshingId === player.id}
              />
            ))}
          </div>

          {/* Pagination */}
          {totalPages > 1 && (
            <div className="flex items-center justify-center gap-2 pt-4">
              <button
                onClick={() => setPage((p) => Math.max(1, p - 1))}
                disabled={page === 1}
                className="rounded-md border px-4 py-2 text-sm font-medium hover:bg-muted disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
              >
                Previous
              </button>
              <span className="text-sm text-muted-foreground">
                Page {page} of {totalPages}
              </span>
              <button
                onClick={() => setPage((p) => Math.min(totalPages, p + 1))}
                disabled={page === totalPages}
                className="rounded-md border px-4 py-2 text-sm font-medium hover:bg-muted disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
              >
                Next
              </button>
            </div>
          )}
        </>
      )}
    </div>
  );
}
