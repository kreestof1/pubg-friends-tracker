import { LucideIcon } from 'lucide-react';
import { cn } from '@/lib/utils';

interface MetricCardProps {
  icon: LucideIcon;
  label: string;
  value: string | number;
  trend?: {
    value: number;
    isPositive: boolean;
  };
  variant?: 'default' | 'primary' | 'success' | 'warning' | 'danger';
  className?: string;
}

export function MetricCard({
  icon: Icon,
  label,
  value,
  trend,
  variant = 'default',
  className,
}: MetricCardProps) {
  const variants = {
    default: 'border-gray-200 bg-white dark:border-gray-800 dark:bg-gray-900',
    primary: 'border-blue-200 bg-blue-50 dark:border-blue-900 dark:bg-blue-950',
    success: 'border-green-200 bg-green-50 dark:border-green-900 dark:bg-green-950',
    warning: 'border-yellow-200 bg-yellow-50 dark:border-yellow-900 dark:bg-yellow-950',
    danger: 'border-red-200 bg-red-50 dark:border-red-900 dark:bg-red-950',
  };

  const iconColors = {
    default: 'text-gray-600 dark:text-gray-400',
    primary: 'text-blue-600 dark:text-blue-400',
    success: 'text-green-600 dark:text-green-400',
    warning: 'text-yellow-600 dark:text-yellow-400',
    danger: 'text-red-600 dark:text-red-400',
  };

  return (
    <div
      className={cn(
        'relative overflow-hidden rounded-lg border p-6 shadow-sm transition-all hover:shadow-md',
        variants[variant],
        className
      )}
    >
      {/* Icon */}
      <div className="flex items-start justify-between">
        <div className={cn('rounded-full bg-white/50 p-3 dark:bg-black/20', iconColors[variant])}>
          <Icon className="h-6 w-6" />
        </div>
        {trend && (
          <div
            className={cn(
              'text-sm font-medium',
              trend.isPositive ? 'text-green-600' : 'text-red-600'
            )}
          >
            {trend.isPositive ? '+' : ''}
            {trend.value}%
          </div>
        )}
      </div>

      {/* Content */}
      <div className="mt-4">
        <p className="text-sm font-medium text-muted-foreground">{label}</p>
        <p className="mt-2 text-3xl font-bold">{value}</p>
      </div>

      {/* Background decoration */}
      <div className="absolute -right-4 -top-4 h-24 w-24 rounded-full bg-white/10 blur-2xl" />
    </div>
  );
}
