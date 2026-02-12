import { render, screen, fireEvent } from '@testing-library/react';
import { ErrorAlert } from '@/components/error-alert';

describe('ErrorAlert', () => {
  it('renders error message', () => {
    render(<ErrorAlert message="Test error message" />);
    expect(screen.getByText('Test error message')).toBeInTheDocument();
  });

  it('renders with default title', () => {
    render(<ErrorAlert message="Error occurred" />);
    expect(screen.getByText('Error')).toBeInTheDocument();
  });

  it('renders with custom title', () => {
    render(<ErrorAlert message="Error occurred" title="Custom Error" />);
    expect(screen.getByText('Custom Error')).toBeInTheDocument();
  });

  it('calls onRetry when retry button is clicked', () => {
    const onRetry = jest.fn();
    render(<ErrorAlert message="Error occurred" onRetry={onRetry} />);
    
    const retryButton = screen.getByRole('button', { name: /try again/i });
    fireEvent.click(retryButton);
    
    expect(onRetry).toHaveBeenCalledTimes(1);
  });

  it('does not render retry button when onRetry is not provided', () => {
    render(<ErrorAlert message="Error occurred" />);
    const retryButton = screen.queryByRole('button', { name: /try again/i });
    expect(retryButton).not.toBeInTheDocument();
  });

  it('renders alert icon', () => {
    const { container } = render(<ErrorAlert message="Error occurred" />);
    const icon = container.querySelector('svg');
    expect(icon).toBeInTheDocument();
  });
});
