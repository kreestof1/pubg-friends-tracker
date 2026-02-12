import { render, screen } from '@testing-library/react';
import { MetricCard } from '@/components/metric-card';
import { Target } from 'lucide-react';

describe('MetricCard', () => {
  it('renders label and value', () => {
    render(
      <MetricCard
        icon={Target}
        label="Kills"
        value={42}
      />
    );
    
    expect(screen.getByText('Kills')).toBeInTheDocument();
    expect(screen.getByText('42')).toBeInTheDocument();
  });

  it('renders with string value', () => {
    render(
      <MetricCard
        icon={Target}
        label="Win Rate"
        value="45.5%"
      />
    );
    
    expect(screen.getByText('Win Rate')).toBeInTheDocument();
    expect(screen.getByText('45.5%')).toBeInTheDocument();
  });

  it('renders with primary variant', () => {
    const { container } = render(
      <MetricCard
        icon={Target}
        label="K/D Ratio"
        value="2.5"
        variant="primary"
      />
    );
    
    const card = container.firstChild;
    expect(card).toHaveClass('border-blue-200');
  });

  it('renders with success variant', () => {
    const { container } = render(
      <MetricCard
        icon={Target}
        label="Win Rate"
        value="50%"
        variant="success"
      />
    );
    
    const card = container.firstChild;
    expect(card).toHaveClass('border-green-200');
  });

  it('renders with trend when provided', () => {
    render(
      <MetricCard
        icon={Target}
        label="Kills"
        value={42}
        trend={{ value: 10, isPositive: true }}
      />
    );
    
    expect(screen.getByText('+10%')).toBeInTheDocument();
  });

  it('renders negative trend', () => {
    render(
      <MetricCard
        icon={Target}
        label="Deaths"
        value={20}
        trend={{ value: -5, isPositive: false }}
      />
    );
    
    expect(screen.getByText('-5%')).toBeInTheDocument();
  });

  it('renders icon', () => {
    const { container } = render(
      <MetricCard
        icon={Target}
        label="Test"
        value={10}
      />
    );
    
    const icon = container.querySelector('svg');
    expect(icon).toBeInTheDocument();
  });
});
