'use client';

import { useState, FormEvent } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';
import { addPlayer } from '@/lib/api';
import { ArrowLeft, Plus, AlertCircle, CheckCircle } from 'lucide-react';
import { LoadingSpinner } from '@/components/loading-spinner';
import type { Shard } from '@/lib/types';

export default function NewPlayerPage() {
  const router = useRouter();
  const [name, setName] = useState('');
  const [shard, setShard] = useState<Shard>('steam');
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState(false);

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setError(null);
    setSuccess(false);

    // Validation
    if (!name.trim()) {
      setError('Player name is required');
      return;
    }

    if (name.length < 3) {
      setError('Player name must be at least 3 characters');
      return;
    }

    if (name.length > 50) {
      setError('Player name must be less than 50 characters');
      return;
    }

    setIsSubmitting(true);

    try {
      const newPlayer = await addPlayer(name.trim(), shard);
      setSuccess(true);
      
      // Redirect after 2 seconds
      setTimeout(() => {
        router.push(`/players/${newPlayer.id}`);
      }, 2000);
    } catch (err) {
      if (err instanceof Error) {
        if (err.message.includes('409')) {
          setError('A player with this name already exists');
        } else if (err.message.includes('404')) {
          setError('Player not found on PUBG servers. Check the name and platform.');
        } else if (err.message.includes('Network')) {
          setError('Network error. Please check your connection and try again.');
        } else {
          setError(err.message || 'Failed to create player. Please try again.');
        }
      } else {
        setError('An unexpected error occurred. Please try again.');
      }
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="container mx-auto p-6 max-w-2xl">
      {/* Header */}
      <div className="flex items-center gap-4 mb-6">
        <Link
          href="/players"
          className="rounded-md p-2 hover:bg-muted transition-colors"
        >
          <ArrowLeft className="w-5 h-5" />
        </Link>
        <div>
          <h1 className="text-4xl font-bold">Add New Player</h1>
          <p className="mt-2 text-muted-foreground">
            Track a new PUBG player's statistics
          </p>
        </div>
      </div>

      {/* Form */}
      <div className="rounded-lg border bg-card p-6">
        <form onSubmit={handleSubmit} className="space-y-6">
          {/* Player Name Input */}
          <div>
            <label htmlFor="name" className="block text-sm font-medium mb-2">
              Player Name <span className="text-red-500">*</span>
            </label>
            <input
              id="name"
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="Enter PUBG player name..."
              disabled={isSubmitting || success}
              className="w-full px-4 py-2 rounded-md border bg-background focus:outline-none focus:ring-2 focus:ring-primary disabled:opacity-50 disabled:cursor-not-allowed"
              autoFocus
            />
            <p className="mt-2 text-sm text-muted-foreground">
              Enter the exact PUBG in-game name (case-sensitive)
            </p>
          </div>

          {/* Platform/Shard Select */}
          <div>
            <label htmlFor="shard" className="block text-sm font-medium mb-2">
              Platform <span className="text-red-500">*</span>
            </label>
            <select
              id="shard"
              value={shard}
              onChange={(e) => setShard(e.target.value as Shard)}
              disabled={isSubmitting || success}
              className="w-full px-4 py-2 rounded-md border bg-background focus:outline-none focus:ring-2 focus:ring-primary disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <option value="steam">Steam (PC)</option>
              <option value="xbox">Xbox</option>
              <option value="psn">PlayStation</option>
              <option value="kakao">Kakao (Korea)</option>
            </select>
            <p className="mt-2 text-sm text-muted-foreground">
              Select the platform where this player plays
            </p>
          </div>

          {/* Error Alert */}
          {error && (
            <div className="flex items-start gap-3 p-4 rounded-md bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800">
              <AlertCircle className="w-5 h-5 text-red-600 dark:text-red-400 flex-shrink-0 mt-0.5" />
              <div className="flex-1">
                <p className="text-sm font-medium text-red-800 dark:text-red-200">
                  {error}
                </p>
              </div>
            </div>
          )}

          {/* Success Alert */}
          {success && (
            <div className="flex items-start gap-3 p-4 rounded-md bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800">
              <CheckCircle className="w-5 h-5 text-green-600 dark:text-green-400 flex-shrink-0 mt-0.5" />
              <div className="flex-1">
                <p className="text-sm font-medium text-green-800 dark:text-green-200">
                  Player added successfully! Redirecting...
                </p>
              </div>
            </div>
          )}

          {/* Submit Button */}
          <div className="flex items-center gap-3">
            <button
              type="submit"
              disabled={isSubmitting || success || !name.trim()}
              className="inline-flex items-center gap-2 rounded-md bg-primary px-6 py-3 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            >
              {isSubmitting ? (
                <>
                  <LoadingSpinner size="sm" />
                  Creating...
                </>
              ) : success ? (
                <>
                  <CheckCircle className="w-4 h-4" />
                  Player Added
                </>
              ) : (
                <>
                  <Plus className="w-4 h-4" />
                  Add Player
                </>
              )}
            </button>
            <Link
              href="/players"
              className="inline-flex items-center gap-2 rounded-md border px-6 py-3 text-sm font-medium hover:bg-muted transition-colors"
            >
              Cancel
            </Link>
          </div>
        </form>

        {/* Info Box */}
        <div className="mt-6 p-4 rounded-md bg-muted">
          <h3 className="text-sm font-semibold mb-2">Important Information</h3>
          <ul className="text-sm text-muted-foreground space-y-1 list-disc list-inside">
            <li>Player name must match exactly (case-sensitive)</li>
            <li>Player must exist on the selected platform</li>
            <li>Initial data fetch may take a few seconds</li>
            <li>Stats are updated when you refresh from the player's page</li>
          </ul>
        </div>
      </div>
    </div>
  );
}
