import { AlertCircle } from 'lucide-react';
import { cn } from '@/lib/utils';

interface ErrorAlertProps {
  title?: string;
  message: string;
  className?: string;
  onRetry?: () => void;
}

export function ErrorAlert({
  title = 'Error',
  message,
  className,
  onRetry,
}: ErrorAlertProps) {
  return (
    <div
      className={cn(
        'rounded-lg border border-red-200 bg-red-50 p-4 dark:border-red-800 dark:bg-red-900/20',
        className
      )}
      role="alert"
    >
      <div className="flex items-start gap-3">
        <AlertCircle className="h-5 w-5 text-red-600 dark:text-red-400" />
        <div className="flex-1">
          <h3 className="font-semibold text-red-800 dark:text-red-300">{title}</h3>
          <p className="mt-1 text-sm text-red-700 dark:text-red-400">{message}</p>
          {onRetry && (
            <button
              onClick={onRetry}
              className="mt-3 text-sm font-medium text-red-800 underline hover:no-underline dark:text-red-300"
            >
              Try again
            </button>
          )}
        </div>
      </div>
    </div>
  );
}
