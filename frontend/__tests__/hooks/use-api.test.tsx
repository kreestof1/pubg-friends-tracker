import { renderHook, waitFor } from '@testing-library/react';
import { usePlayers, usePlayer, useDashboard, usePlayerStats } from '@/hooks/use-api';
import * as api from '@/lib/api';

// Mock the API module
jest.mock('@/lib/api');

// Mock SWR
jest.mock('swr', () => {
  const actualSWR = jest.requireActual('swr');
  return {
    __esModule: true,
    default: jest.fn((key, fetcher, config) => {
      const mockData = {
        '/players?page=1&limit=10': [
          { id: '1', name: 'Player1', shard: 'steam' },
          { id: '2', name: 'Player2', shard: 'steam' },
        ],
        '/players/123': { id: '123', name: 'TestPlayer', shard: 'steam' },
        '/dashboard?ids=1,2&period=7d&mode=solo&shard=steam': {
          players: [
            { player_id: '1', name: 'Player1', stats: {} },
            { player_id: '2', name: 'Player2', stats: {} },
          ],
        },
      };

      // Handle player stats keys with pattern matching
      if (key && typeof key === 'string' && key.includes('/players/') && key.includes('/stats')) {
        return {
          data: {
            player_id: '123',
            kills: 50,
            deaths: 20,
            kd_ratio: 2.5,
            win_rate: 30.0,
          },
          error: undefined,
          isLoading: false,
          mutate: jest.fn(),
        };
      }

      return {
        data: key ? mockData[key] : undefined,
        error: undefined,
        isLoading: false,
        mutate: jest.fn(),
      };
    }),
  };
});

describe('useApi hooks', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  describe('usePlayers', () => {
    it('fetches players list', async () => {
      const { result } = renderHook(() => usePlayers(1, 10));

      await waitFor(() => {
        expect(result.current.players).toBeDefined();
        expect(result.current.isLoading).toBe(false);
      });

      expect(result.current.players).toHaveLength(2);
      expect(result.current.players?.[0]).toHaveProperty('name', 'Player1');
    });

    it('provides mutate function', () => {
      const { result } = renderHook(() => usePlayers());

      expect(result.current.mutate).toBeDefined();
      expect(typeof result.current.mutate).toBe('function');
    });
  });

  describe('usePlayer', () => {
    it('fetches single player by id', async () => {
      const { result } = renderHook(() => usePlayer('123'));

      await waitFor(() => {
        expect(result.current.player).toBeDefined();
      });

      expect(result.current.player).toHaveProperty('name', 'TestPlayer');
      expect(result.current.player).toHaveProperty('id', '123');
    });

    it('returns null when id is null', () => {
      const { result } = renderHook(() => usePlayer(null));

      expect(result.current.player).toBeUndefined();
    });
  });

  describe('useDashboard', () => {
    it('fetches dashboard data for selected players', async () => {
      const { result } = renderHook(() =>
        useDashboard(['1', '2'], '7d', 'solo', 'steam')
      );

      await waitFor(() => {
        expect(result.current.dashboard).toBeDefined();
      });

      expect(result.current.dashboard?.players).toHaveLength(2);
    });

    it('returns null when no players selected', () => {
      const { result } = renderHook(() =>
        useDashboard([], '7d', 'solo', 'steam')
      );

      expect(result.current.dashboard).toBeUndefined();
    });
  });

  describe('usePlayerStats', () => {
    it('fetches player stats with filters', () => {
      const { result } = renderHook(() =>
        usePlayerStats('123', '7d', 'solo', 'steam')
      );

      expect(result.current.stats).toBeDefined();
    });

    it('returns null when playerId is null', () => {
      const { result } = renderHook(() =>
        usePlayerStats(null, '7d', 'solo', 'steam')
      );

      expect(result.current.stats).toBeUndefined();
    });
  });
});
