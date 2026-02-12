import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import PlayersPage from '@/app/players/page';

// Mock Next.js router
const mockPush = jest.fn();
jest.mock('next/navigation', () => ({
  useRouter: () => ({
    push: mockPush,
    refresh: jest.fn(),
  }),
}));

// Mock Next.js Link
jest.mock('next/link', () => {
  return ({ children, href, ...props }: any) => {
    return <a href={href} {...props}>{children}</a>;
  };
});

// Mock SWR
jest.mock('swr', () => {
  return jest.fn(() => ({
    data: [
      {
        id: '1',
        account_id: 'acc1',
        name: 'TestPlayer1',
        shard: 'steam',
        last_matches: ['m1', 'm2', 'm3'],
        last_refreshed_at: new Date('2026-02-09'),
        created_at: new Date('2026-01-01'),
      },
      {
        id: '2',
        account_id: 'acc2',
        name: 'TestPlayer2',
        shard: 'xbox',
        last_matches: ['m4', 'm5'],
        last_refreshed_at: new Date('2026-02-08'),
        created_at: new Date('2026-01-02'),
      },
      {
        id: '3',
        account_id: 'acc3',
        name: 'AnotherPlayer',
        shard: 'steam',
        last_matches: ['m6'],
        last_refreshed_at: new Date('2026-02-07'),
        created_at: new Date('2026-01-03'),
      },
    ],
    error: undefined,
    isLoading: false,
    mutate: jest.fn(),
  }));
});

describe('Players Page Integration Tests', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('renders players page with title', async () => {
    render(<PlayersPage />);

    await waitFor(() => {
      expect(screen.getByText('Players')).toBeInTheDocument();
    });

    expect(screen.getByText('Manage and track all your PUBG friends')).toBeInTheDocument();
  });

  it('displays all players', async () => {
    render(<PlayersPage />);

    await waitFor(() => {
      expect(screen.getByText('TestPlayer1')).toBeInTheDocument();
    });

    expect(screen.getByText('TestPlayer2')).toBeInTheDocument();
    expect(screen.getByText('AnotherPlayer')).toBeInTheDocument();
  });

  it('has Add Player button with correct link', async () => {
    render(<PlayersPage />);

    await waitFor(() => {
      expect(screen.getByText('Add Player')).toBeInTheDocument();
    });

    const addButton = screen.getByText('Add Player').closest('a');
    expect(addButton).toHaveAttribute('href', '/players/new');
  });

  it('filters players by search query', async () => {
    render(<PlayersPage />);

    await waitFor(() => {
      expect(screen.getByText('TestPlayer1')).toBeInTheDocument();
    });

    const searchInput = screen.getByPlaceholderText('Search players by name...');
    fireEvent.change(searchInput, { target: { value: 'Another' } });

    await waitFor(() => {
      expect(screen.getByText('AnotherPlayer')).toBeInTheDocument();
      expect(screen.queryByText('TestPlayer1')).not.toBeInTheDocument();
      expect(screen.queryByText('TestPlayer2')).not.toBeInTheDocument();
    });
  });

  it('filters players by shard', async () => {
    render(<PlayersPage />);

    await waitFor(() => {
      expect(screen.getByText('TestPlayer1')).toBeInTheDocument();
    });

    const shardFilter = screen.getByRole('combobox');
    fireEvent.change(shardFilter, { target: { value: 'xbox' } });

    await waitFor(() => {
      expect(screen.getByText('TestPlayer2')).toBeInTheDocument();
      expect(screen.queryByText('TestPlayer1')).not.toBeInTheDocument();
      expect(screen.queryByText('AnotherPlayer')).not.toBeInTheDocument();
    });
  });

  it('combines search and shard filters', async () => {
    render(<PlayersPage />);

    await waitFor(() => {
      expect(screen.getByText('TestPlayer1')).toBeInTheDocument();
    });

    const searchInput = screen.getByPlaceholderText('Search players by name...');
    fireEvent.change(searchInput, { target: { value: 'Test' } });

    const shardFilter = screen.getByRole('combobox');
    fireEvent.change(shardFilter, { target: { value: 'steam' } });

    await waitFor(() => {
      expect(screen.getByText('TestPlayer1')).toBeInTheDocument();
      expect(screen.queryByText('TestPlayer2')).not.toBeInTheDocument();
      expect(screen.queryByText('AnotherPlayer')).not.toBeInTheDocument();
    });
  });

  it('shows empty state when no players match filters', async () => {
    render(<PlayersPage />);

    await waitFor(() => {
      expect(screen.getByText('TestPlayer1')).toBeInTheDocument();
    });

    const searchInput = screen.getByPlaceholderText('Search players by name...');
    fireEvent.change(searchInput, { target: { value: 'NonExistentPlayer' } });

    await waitFor(() => {
      expect(screen.getByText('No players found')).toBeInTheDocument();
      expect(screen.getByText('Try adjusting your search or filters')).toBeInTheDocument();
    });
  });

  it('displays player cards with correct information', async () => {
    render(<PlayersPage />);

    await waitFor(() => {
      expect(screen.getByText('TestPlayer1')).toBeInTheDocument();
    });

    expect(screen.getAllByText('STEAM')).toHaveLength(2);
    expect(screen.getByText('XBOX')).toBeInTheDocument();
  });

  it('clears search input', async () => {
    render(<PlayersPage />);

    await waitFor(() => {
      expect(screen.getByText('TestPlayer1')).toBeInTheDocument();
    });

    const searchInput = screen.getByPlaceholderText('Search players by name...') as HTMLInputElement;
    fireEvent.change(searchInput, { target: { value: 'Test' } });

    expect(searchInput.value).toBe('Test');

    fireEvent.change(searchInput, { target: { value: '' } });

    expect(searchInput.value).toBe('');
    
    await waitFor(() => {
      expect(screen.getByText('TestPlayer1')).toBeInTheDocument();
      expect(screen.getByText('TestPlayer2')).toBeInTheDocument();
      expect(screen.getByText('AnotherPlayer')).toBeInTheDocument();
    });
  });
});
