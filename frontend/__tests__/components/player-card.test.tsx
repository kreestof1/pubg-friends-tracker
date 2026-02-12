import { render, screen, fireEvent } from '@testing-library/react';
import { PlayerCard } from '@/components/player-card';
import type { Player } from '@/lib/types';

// Mock Next.js Link component
jest.mock('next/link', () => {
  return ({ children, href, ...props }: any) => {
    return <a href={href} {...props}>{children}</a>;
  };
});

const mockPlayer: Player = {
  id: '123',
  account_id: 'account-123',
  name: 'TestPlayer',
  shard: 'steam',
  last_matches: ['match1', 'match2', 'match3'],
  last_refreshed_at: new Date('2026-02-09'),
  created_at: new Date('2026-01-01'),
};

describe('PlayerCard', () => {
  it('renders player name and shard', () => {
    render(<PlayerCard player={mockPlayer} />);
    
    expect(screen.getByText('TestPlayer')).toBeInTheDocument();
    expect(screen.getByText('STEAM')).toBeInTheDocument();
  });

  it('displays match count', () => {
    render(<PlayerCard player={mockPlayer} />);
    
    expect(screen.getByText('3')).toBeInTheDocument();
    expect(screen.getByText('Matches:')).toBeInTheDocument();
  });

  it('displays formatted last refresh date', () => {
    render(<PlayerCard player={mockPlayer} />);
    
    expect(screen.getByText(/feb/i)).toBeInTheDocument();
  });

  it('displays "Never" when last_refreshed_at is undefined', () => {
    const playerWithoutRefresh = { ...mockPlayer, last_refreshed_at: undefined };
    render(<PlayerCard player={playerWithoutRefresh} />);
    
    expect(screen.getByText('Never')).toBeInTheDocument();
  });

  it('shows "0" matches when last_matches is undefined', () => {
    const playerWithoutMatches = { ...mockPlayer, last_matches: undefined };
    render(<PlayerCard player={playerWithoutMatches} />);
    
    expect(screen.getByText('0')).toBeInTheDocument();
  });

  it('renders refresh and view details buttons when showActions is true', () => {
    render(<PlayerCard player={mockPlayer} showActions={true} />);
    
    expect(screen.getByText('Refresh')).toBeInTheDocument();
    expect(screen.getByText('View Details')).toBeInTheDocument();
  });

  it('hides action buttons when showActions is false', () => {
    render(<PlayerCard player={mockPlayer} showActions={false} />);
    
    expect(screen.queryByText('Refresh')).not.toBeInTheDocument();
    expect(screen.queryByText('View Details')).not.toBeInTheDocument();
  });

  it('calls onRefresh when refresh button is clicked', () => {
    const onRefresh = jest.fn();
    render(<PlayerCard player={mockPlayer} onRefresh={onRefresh} />);
    
    const refreshButton = screen.getByText('Refresh');
    fireEvent.click(refreshButton);
    
    expect(onRefresh).toHaveBeenCalledTimes(1);
  });

  it('calls onClick when card is clicked', () => {
    const onClick = jest.fn();
    render(<PlayerCard player={mockPlayer} onClick={onClick} />);
    
    const card = screen.getByText('TestPlayer').closest('div')?.parentElement;
    if (card) {
      fireEvent.click(card);
    }
    
    expect(onClick).toHaveBeenCalledTimes(1);
  });

  it('has correct link href for View Details button', () => {
    render(<PlayerCard player={mockPlayer} />);
    
    const viewDetailsLink = screen.getByText('View Details').closest('a');
    expect(viewDetailsLink).toHaveAttribute('href', '/players/123');
  });

  it('stops propagation when clicking refresh button', () => {
    const onClick = jest.fn();
    const onRefresh = jest.fn();
    render(<PlayerCard player={mockPlayer} onClick={onClick} onRefresh={onRefresh} />);
    
    const refreshButton = screen.getByText('Refresh');
    fireEvent.click(refreshButton);
    
    expect(onRefresh).toHaveBeenCalledTimes(1);
    expect(onClick).not.toHaveBeenCalled();
  });

  it('applies custom className', () => {
    const { container } = render(<PlayerCard player={mockPlayer} className="custom-class" />);
    
    const card = container.firstChild;
    expect(card).toHaveClass('custom-class');
  });
});
