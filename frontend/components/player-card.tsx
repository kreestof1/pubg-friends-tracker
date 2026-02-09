import { User, Calendar, Target } from 'lucide-react';
import { cn } from '@/lib/utils';
import type { Player } from '@/lib/types';

interface PlayerCardProps {
  player: Player;
  onClick?: () => void;
  onRefresh?: () => void;
  className?: string;
  showActions?: boolean;
}

export function PlayerCard({
  player,
  onClick,
  onRefresh,
  className,
  showActions = true,
}: PlayerCardProps) {
  const formatDate = (date?: Date) => {
    if (!date) return 'Never';
    return new Date(date).toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
    });
  };

  return (
    <div
      className={cn(
        'group relative overflow-hidden rounded-lg border bg-card p-6 shadow-sm transition-all hover:shadow-md',
        onClick && 'cursor-pointer hover:border-primary',
        className
      )}
      onClick={onClick}
    >
      {/* Header */}
      <div className="flex items-start justify-between">
        <div className="flex items-center gap-3">
          <div className="rounded-full bg-primary/10 p-3">
            <User className="h-6 w-6 text-primary" />
          </div>
          <div>
            <h3 className="font-semibold text-lg">{player.name}</h3>
            <p className="text-sm text-muted-foreground">
              {player.shard.toUpperCase()}
            </p>
          </div>
        </div>
      </div>

      {/* Stats */}
      <div className="mt-4 grid grid-cols-2 gap-4">
        <div className="flex items-center gap-2 text-sm">
          <Target className="h-4 w-4 text-muted-foreground" />
          <span className="text-muted-foreground">Matches:</span>
          <span className="font-medium">{player.last_matches?.length || 0}</span>
        </div>
        <div className="flex items-center gap-2 text-sm">
          <Calendar className="h-4 w-4 text-muted-foreground" />
          <span className="text-muted-foreground">Updated:</span>
          <span className="font-medium">
            {formatDate(player.last_refreshed_at)}
          </span>
        </div>
      </div>

      {/* Actions */}
      {showActions && (
        <div className="mt-4 flex gap-2">
          <button
            onClick={(e) => {
              e.stopPropagation();
              onRefresh?.();
            }}
            className="flex-1 rounded-md border border-primary bg-primary/10 px-4 py-2 text-sm font-medium text-primary transition-colors hover:bg-primary/20"
          >
            Refresh
          </button>
          <button
            onClick={(e) => {
              e.stopPropagation();
              onClick?.();
            }}
            className="flex-1 rounded-md border bg-background px-4 py-2 text-sm font-medium transition-colors hover:bg-muted"
          >
            View Details
          </button>
        </div>
      )}

      {/* Gradient overlay on hover */}
      <div className="absolute inset-0 bg-gradient-to-r from-primary/5 to-transparent opacity-0 transition-opacity group-hover:opacity-100" />
    </div>
  );
}
