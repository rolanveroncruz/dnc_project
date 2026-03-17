import {
    Component,
    forwardRef,
    input,
    signal,
    computed,
    effect,
    ViewChild,
    ElementRef,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { ControlValueAccessor, NG_VALUE_ACCESSOR, ReactiveFormsModule } from '@angular/forms';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatHint, MatLabel } from '@angular/material/input';

type NullableNumber = number | null;

@Component({
    selector: 'app-currency-input',
    standalone: true,
    imports: [CommonModule, ReactiveFormsModule, MatFormFieldModule, MatInputModule, MatLabel, MatHint],
    templateUrl: './currency-input-component.html',
    styleUrl: './currency-input-component.scss',
    providers: [
        {
            provide: NG_VALUE_ACCESSOR,
            useExisting: forwardRef(() => CurrencyInputComponent),
            multi: true,
        },
    ],
})
export class CurrencyInputComponent implements ControlValueAccessor {
    // ---- Inputs
    readonly label = input<string>('Amount');
    readonly placeholder = input<string>('0.00');
    readonly hint = input<string>('');
    readonly required = input<boolean>(false);
    readonly appearance = input<'fill' | 'outline'>('outline');

    /**
     * Currency code only affects formatting style. (e.g., USD/PHP)
     * We DON'T print the symbol by default; we just format grouping/decimals.
     */
    readonly currency = input<string>('PHP');

    @ViewChild('textInput', { static: true }) textInput!: ElementRef<HTMLInputElement>;

    // ---- Internal state
    private readonly disabledSig = signal(false);
    private readonly displaySig = signal<string>(''); // what user sees
    private readonly valueSig = signal<NullableNumber>(null); // number emitted to parent

    // CVA callbacks
    private onChange: (v: NullableNumber) => void = () => {};
    private onTouched: () => void = () => {};

    // Use Intl to compute group/decimal separators for locale.
    // We'll default to en-US style commas+dot per your request (commas + decimal point).
    private readonly locale = 'en-US';

    // ---- CVA
    writeValue(value: NullableNumber): void {
        this.valueSig.set(this.coerceToNumberOrNull(value));
        this.displaySig.set(this.valueSig() == null ? '' : this.formatFixed2(this.valueSig()!));
    }

    registerOnChange(fn: (v: NullableNumber) => void): void {
        this.onChange = fn;
    }

    registerOnTouched(fn: () => void): void {
        this.onTouched = fn;
    }

    setDisabledState(isDisabled: boolean): void {
        this.disabledSig.set(isDisabled);
    }

    // ---- Template bindings
    readonly displayValue = computed(() => this.displaySig());
    readonly isDisabled = computed(() => this.disabledSig());

    // ---- Events
    onInput(e: Event): void {
        const el = e.target as HTMLInputElement;

        // Save caret before we mutate value
        const oldRaw = el.value ?? '';
        const oldCaret = el.selectionStart ?? oldRaw.length;

        // Parse digits/decimal, clamp to 2 decimals, produce numeric value
        const parsed = this.parseUserText(oldRaw);

        // If user cleared, propagate null
        if (parsed == null) {
            this.valueSig.set(null);
            this.displaySig.set('');
            this.onChange(null);
            // caret stays near start
            queueMicrotask(() => this.setCaret(el, Math.min(oldCaret, el.value.length)));
            return;
        }

        // Update model
        this.valueSig.set(parsed);
        this.onChange(parsed);

        // Format display as 1,234.56
        const formatted = this.formatFixed2(parsed);
        this.displaySig.set(formatted);

        // Update actual input DOM value and attempt to preserve caret.
        // Approach: keep caret anchored relative to digits to the left.
        const digitsLeft = this.countDigitsLeft(oldRaw, oldCaret);
        queueMicrotask(() => {
            el.value = formatted;
            const newCaret = this.caretFromDigitsLeft(formatted, digitsLeft);
            this.setCaret(el, newCaret);
        });
    }

    onBlur(): void {
        this.onTouched();

        // On blur, normalize display to fixed 2 decimals (already is),
        // but if empty keep empty.
        const v = this.valueSig();
        if (v == null) {
            this.displaySig.set('');
            return;
        }
        this.displaySig.set(this.formatFixed2(v));
    }

    // ---- Helpers

    private coerceToNumberOrNull(v: any): NullableNumber {
        if (v === '' || v === undefined || v === null) return null;
        const n = Number(v);
        return Number.isFinite(n) ? n : null;
    }

    /**
     * Parse user text that may contain commas, currency symbols, spaces.
     * Keeps only digits and at most one '.'.
     * Clamps to 2 decimal places.
     *
     * Returns null if no digits.
     */
    private parseUserText(text: string): NullableNumber {
        if (!text) return null;

        // Keep digits and dots, remove commas and others
        let cleaned = text.replace(/,/g, '').replace(/[^\d.]/g, '');

        // If multiple dots, keep first and drop rest
        const firstDot = cleaned.indexOf('.');
        if (firstDot !== -1) {
            const before = cleaned.slice(0, firstDot);
            const after = cleaned.slice(firstDot + 1).replace(/\./g, '');
            cleaned = `${before}.${after}`;
        }

        // If nothing numeric
        if (!/\d/.test(cleaned)) return null;

        // Split and clamp decimals
        const [intPartRaw, decPartRaw = ''] = cleaned.split('.');
        const intPart = intPartRaw.replace(/^0+(?=\d)/, ''); // avoid "00012"
        const decPart = decPartRaw.slice(0, 2);

        const normalized = decPart.length > 0 ? `${intPart || '0'}.${decPart}` : `${intPart || '0'}`;

        const n = Number(normalized);
        if (!Number.isFinite(n)) return null;

        // Round to 2 decimals (prevents floating weirdness for large edits)
        return Math.round(n * 100) / 100;
    }

    private formatFixed2(value: number): string {
        // Force 2 decimals, group by commas, decimal point.
        return new Intl.NumberFormat(this.locale, {
            minimumFractionDigits: 2,
            maximumFractionDigits: 2,
            useGrouping: true,
        }).format(value);
    }

    private setCaret(el: HTMLInputElement, pos: number): void {
        try {
            el.setSelectionRange(pos, pos);
        } catch {
            // ignore (some inputs/browsers can throw)
        }
    }

    private countDigitsLeft(text: string, caretPos: number): number {
        const left = text.slice(0, Math.max(0, caretPos));
        const digits = left.replace(/[^\d]/g, '');
        return digits.length;
    }

    private caretFromDigitsLeft(formatted: string, digitsLeft: number): number {
        if (digitsLeft <= 0) return 0;

        let count = 0;
        for (let i = 0; i < formatted.length; i++) {
            if (/\d/.test(formatted[i])) count++;
            if (count >= digitsLeft) {
                // place caret just after this digit
                return i + 1;
            }
        }
        return formatted.length;
    }
}
