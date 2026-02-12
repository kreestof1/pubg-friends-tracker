import { cn } from '@/lib/utils';

describe('Utils', () => {
  describe('cn (className utility)', () => {
    it('merges class names', () => {
      const result = cn('px-4', 'py-2');
      expect(result).toBe('px-4 py-2');
    });

    it('handles conditional classes', () => {
      const result = cn('base-class', true && 'true-class', false && 'false-class');
      expect(result).toBe('base-class true-class');
    });

    it('merges Tailwind classes correctly', () => {
      const result = cn('px-2', 'px-4');
      expect(result).toBe('px-4');
    });

    it('handles arrays of classes', () => {
      const result = cn(['class1', 'class2'], 'class3');
      expect(result).toBe('class1 class2 class3');
    });

    it('handles objects with conditional classes', () => {
      const result = cn({
        'class1': true,
        'class2': false,
        'class3': true,
      });
      expect(result).toBe('class1 class3');
    });

    it('handles undefined and null values', () => {
      const result = cn('class1', undefined, null, 'class2');
      expect(result).toBe('class1 class2');
    });

    it('handles empty input', () => {
      const result = cn();
      expect(result).toBe('');
    });

    it('merges conflicting Tailwind classes (last wins)', () => {
      const result = cn('bg-red-500', 'bg-blue-500');
      expect(result).toBe('bg-blue-500');
    });

    it('handles complex scenarios', () => {
      const isActive = true;
      const hasError = false;
      const result = cn(
        'base-class',
        isActive && 'active-class',
        hasError && 'error-class',
        'p-4'
      );
      expect(result).toBe('base-class active-class p-4');
    });
  });
});
