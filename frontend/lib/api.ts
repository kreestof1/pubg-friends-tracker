// API client configuration and functions
import axios from 'axios';

const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080/api';

export const api = axios.create({
  baseURL: API_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Player API functions
export async function addPlayer(name: string, shard: string) {
  const response = await api.post('/players', { name, shard });
  return response.data;
}

export async function getPlayers(page: number = 1, limit: number = 10) {
  const response = await api.get('/players', {
    params: { page, limit },
  });
  return response.data;
}

export async function getPlayer(id: string) {
  const response = await api.get(`/players/${id}`);
  return response.data;
}

export async function refreshPlayer(id: string) {
  const response = await api.post(`/players/${id}/refresh`);
  return response.data;
}

export async function getPlayerMatches(id: string) {
  const response = await api.get(`/players/${id}/matches`);
  return response.data;
}

// Dashboard and statistics API functions
export async function getDashboardStats(
  playerIds: string[],
  period: string,
  mode: string,
  shard: string
) {
  const response = await api.get('/dashboard', {
    params: {
      ids: playerIds.join(','),
      period,
      mode,
      shard,
    },
  });
  return response.data;
}

export async function getPlayerStats(
  playerId: string,
  period: string,
  mode: string,
  shard: string
) {
  const response = await api.get(`/players/${playerId}/stats`, {
    params: { period, mode, shard },
  });
  return response.data;
}
