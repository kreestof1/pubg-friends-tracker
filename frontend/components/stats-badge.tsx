import { cn } from '@/lib/utils';
import { LucideIcon } from 'lucide-react';

interface StatsBadgeProps {
  icon?: LucideIcon;
  label: string;
  value: string | number;
  variant?: 'default' | 'success' | 'warning' | 'danger';
  className?: string;
}

export function StatsBadge({
  icon: Icon,
  label,
  value,
  variant = 'default',
  className,
}: StatsBadgeProps) {
  const variants = {
    default: 'bg-gray-100 text-gray-900 dark:bg-gray-800 dark:text-gray-100',
    success: 'bg-green-100 text-green-900 dark:bg-green-900/20 dark:text-green-300',
    warning: 'bg-yellow-100 text-yellow-900 dark:bg-yellow-900/20 dark:text-yellow-300',
    danger: 'bg-red-100 text-red-900 dark:bg-red-900/20 dark:text-red-300',
  };

  return (
    <div
      className={cn(
        'flex items-center gap-2 rounded-lg p-3',
        variants[variant],
        className
      )}
    >
      {Icon && <Icon className="h-5 w-5" />}
      <div className="flex flex-col">
        <span className="text-xs font-medium opacity-80">{label}</span>
        <span className="text-lg font-bold">{value}</span>
      </div>
    </div>
  );
}
