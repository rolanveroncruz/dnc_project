import { CommonModule } from '@angular/common';
import {
    Component,
    DestroyRef,
    ElementRef,
    EventEmitter,
    Input,
    Output,
    ViewChild,
    computed,
    effect,
    forwardRef,
    inject,
    signal,
} from '@angular/core';
import {
    ControlValueAccessor,
    FormControl,
    NG_VALUE_ACCESSOR,
    ReactiveFormsModule,
} from '@angular/forms';
import { takeUntilDestroyed, toSignal } from '@angular/core/rxjs-interop';

import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import {
    MatAutocompleteModule,
    MatAutocompleteSelectedEvent,
} from '@angular/material/autocomplete';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';
import { MatChipsModule } from '@angular/material/chips';

import { map, startWith } from 'rxjs/operators';

export type AddableAutocompleteItem = {
    id: string;
    label: string;
};

// CVA value type:
// - If showChips === false: the form value is a single id (string | null)
// - If showChips === true : the form value is multiple ids (string[])
export type AddableAutocompleteValue = string | null | string[];

@Component({
    selector: 'app-addable-autocomplete',
    standalone: true,
    imports: [
        CommonModule,
        ReactiveFormsModule,

        MatFormFieldModule,
        MatInputModule,
        MatAutocompleteModule,
        MatIconModule,
        MatButtonModule,
        MatChipsModule,
    ],
    templateUrl: './addable-autocomplete-component.html',
    styleUrl: './addable-autocomplete-component.scss',
    providers: [
        {
            provide: NG_VALUE_ACCESSOR,
            useExisting: forwardRef(() => AddableAutocompleteComponent),
            multi: true,
        },
    ],
})
export class AddableAutocompleteComponent implements ControlValueAccessor {
    private readonly destroyRef = inject(DestroyRef);

    // ---------- Inputs ----------
    @Input() label = 'Select or add';
    @Input() placeholder = 'Type to search…';
    @Input() hint = '';

    /**
     * Optional: if true, we emit/accept multiple ids (string[]) and display chips.
     * If false, we emit/accept a single id (string | null).
     */
    @Input() showChips = true;

    /**
     * If true: selecting an option also adds it to "selected" (chips).
     * If false: selecting just fills input (but still updates value for single-select).
     */
    @Input() selectAddsToChips = true;

    /**
     * The source list (shown in the dropdown).
     */
    @Input({ required: true }) set items(value: AddableAutocompleteItem[]) {
        this._items.set(value ?? []);
    }

    // ---------- Outputs (still useful even as CVA) ----------
    @Output() itemCreated = new EventEmitter<AddableAutocompleteItem>();
    @Output() itemSelected = new EventEmitter<AddableAutocompleteItem>();
    @Output() selectedChange = new EventEmitter<AddableAutocompleteItem[]>();

    @ViewChild('textInput', { static: true }) textInput!: ElementRef<HTMLInputElement>;

    // ---------- Internal State ----------
    readonly ctrl = new FormControl<string | AddableAutocompleteItem>('', { nonNullable: true });

    private readonly _items = signal<AddableAutocompleteItem[]>([]);
    readonly selected = signal<AddableAutocompleteItem[]>([]);
    readonly filteredOptions = signal<AddableAutocompleteItem[]>([]);

    // CVA plumbing
    private _onChange: (v: AddableAutocompleteValue) => void = () => {};
    private _onTouched: () => void = () => {};
    private _disabled = signal(false);

    readonly term = toSignal(
        this.ctrl.valueChanges.pipe(
            startWith(this.ctrl.value),
            map((v) => this.norm(v).trim()),
        ),
        { initialValue: '' },
    );

    // Used to defer applying a writeValue() until items arrive
    private readonly _pendingIds = signal<string[] | null>(null);

    constructor() {
        // Debug guard
        effect(() => {
            const list = this._items();
            const bad = list.filter((x) => typeof (x as any)?.label !== 'string');
            if (bad.length) console.warn('AddableAutocompleteComponent: items must have a label property', bad);
        });

        // Keep filteredOptions in sync with ctrl.value
        this.ctrl.valueChanges
            .pipe(
                startWith(this.ctrl.value),
                map((v) => this.norm(v).trim()),
                takeUntilDestroyed(this.destroyRef),
            )
            .subscribe((term) => {
                const q = term.toLowerCase();
                const list = this._items();
                if (!q) {
                    this.filteredOptions.set(list.slice(0, 50));
                    return;
                }
                this.filteredOptions.set(
                    list
                        .filter((x) => (x.label ?? '').toLowerCase().includes(q))
                        .slice(0, 50),
                );
            });

        // When items change, try to resolve any pending ids from writeValue()
        effect(() => {
            const pending = this._pendingIds();
            const list = this._items();
            if (!pending) return;
            if (!list.length) return;

            this.applyIdsToUI(pending, /*emit*/ false);
            this._pendingIds.set(null);
        });

        // If user manually edits the text in single-select mode, do NOT automatically change the form value.
        // The value should change when they pick an option or create one.
        // (If you want “free text” support, tell me and I’ll wire it.)
    }

    // ---------- CVA ----------
    writeValue(value: AddableAutocompleteValue): void {
        // Normalize incoming value -> ids[]
        const ids = Array.isArray(value) ? value : value ? [value] : [];

        // If items aren’t loaded yet, store pending and apply later.
        if (!this._items().length && ids.length) {
            this._pendingIds.set(ids);
            // Still clear UI in the meantime
            this.applyIdsToUI([], /*emit*/ false);
            return;
        }

        this.applyIdsToUI(ids, /*emit*/ false);
    }

    registerOnChange(fn: (v: AddableAutocompleteValue) => void): void {
        this._onChange = fn;
    }

    registerOnTouched(fn: () => void): void {
        this._onTouched = fn;
    }

    setDisabledState(isDisabled: boolean): void {
        this._disabled.set(isDisabled);
        if (isDisabled) this.ctrl.disable({ emitEvent: false });
        else this.ctrl.enable({ emitEvent: false });
    }

    // ---------- UI helpers ----------
    private asText(v: string | AddableAutocompleteItem | null | undefined): string {
        if (v === null || v === undefined) return '';
        if (typeof v === 'string') return v;
        if (typeof v === 'object' && 'label' in v) {
            const label = (v as any).label;
            return typeof label === 'string' ? label : '';
        }
        return '';
    }

    private norm(s: string | AddableAutocompleteItem | null | undefined): string {
        return this.asText(s).trim().toLowerCase();
    }

    readonly canAddTypedValue = computed(() => {
        const raw = this.asText(this.term());
        const text = raw.trim();
        if (!text) return false;

        const exists = this._items().some((x) => (x.label ?? '').trim().toLowerCase() === text.toLowerCase());
        return !exists;
    });

    // ---------- Actions ----------
    onOptionSelected(ev: MatAutocompleteSelectedEvent) {
        const picked = ev.option.value as AddableAutocompleteItem;
        this.itemSelected.emit(picked);

        if (this.showChips) {
            // multi-select mode
            if (this.selectAddsToChips) {
                this.addToSelected(picked);
                this.ctrl.setValue('', { emitEvent: true });
                this.emitValueChange(); // ids[]
            } else {
                // show label in input but don’t add chip
                this.ctrl.setValue(picked.label, { emitEvent: true });
                // If showChips=true but selectAddsToChips=false, treat as single selection
                this.selected.set([picked]);
                this.emitValueChange(); // ids[]
            }
        } else {
            // single-select mode: selecting sets the value immediately
            this.selected.set([picked]);
            this.ctrl.setValue(picked.label, { emitEvent: true });
            this.emitValueChange(); // id | null
        }

        this.markTouched();
        this.focusInput();
    }

    addTypedValue() {
        const raw = this.ctrl.value;
        const label =
            typeof raw === 'string' ? raw.trim() : (raw?.label ?? '').trim();

        if (!label) return;

        // See if it already exists
        const existing = this._items().find((x) => (x.label ?? '').trim().toLowerCase() === label.toLowerCase());
        if (existing) {
            // treat as selecting existing
            this.itemSelected.emit(existing);

            if (this.showChips) {
                if (this.selectAddsToChips) {
                    this.addToSelected(existing);
                    this.ctrl.setValue('', { emitEvent: true });
                } else {
                    this.selected.set([existing]);
                    this.ctrl.setValue(existing.label, { emitEvent: true });
                }
            } else {
                this.selected.set([existing]);
                this.ctrl.setValue(existing.label, { emitEvent: true });
            }

            this.emitValueChange();
            this.markTouched();
            this.focusInput();
            return;
        }

        // Create new (client-side id). Parent should persist & refresh items with real id.
        const created: AddableAutocompleteItem = {
            id: crypto.randomUUID(),
            label,
        };

        // Update local list so UI can show it immediately
        this._items.set([created, ...this._items()]);
        this.itemCreated.emit(created);

        if (this.showChips) {
            if (this.selectAddsToChips) {
                this.addToSelected(created);
                this.ctrl.setValue('', { emitEvent: true });
            } else {
                this.selected.set([created]);
                this.ctrl.setValue(created.label, { emitEvent: true });
            }
        } else {
            this.selected.set([created]);
            this.ctrl.setValue(created.label, { emitEvent: true });
        }

        this.emitValueChange();
        this.markTouched();
        this.focusInput();
    }

    removeChip(item: AddableAutocompleteItem) {
        const next = this.selected().filter((x) => x.id !== item.id);
        this.selected.set(next);
        this.selectedChange.emit(next);

        this.emitValueChange();
        this.markTouched();
    }

    clearInput() {
        // Clear visible text; for single-select, also clear selection/value.
        this.ctrl.setValue('', { emitEvent: true });

        if (!this.showChips) {
            this.selected.set([]);
            this.emitValueChange(); // null
        }

        this.markTouched();
        this.focusInput();
    }

    onEnterKey() {
        if (this.canAddTypedValue()) this.addTypedValue();
    }

    private addToSelected(item: AddableAutocompleteItem) {
        if (this.selected().some((x) => x.id === item.id)) return;
        const next = [...this.selected(), item];
        this.selected.set(next);
        this.selectedChange.emit(next);
    }

    private focusInput() {
        queueMicrotask(() => this.textInput?.nativeElement?.focus());
    }

    displayWith(x: AddableAutocompleteItem | string | null): string {
        if (typeof x === 'string') return x;
        return x?.label ?? '';
    }

    // ---------- Value mapping ----------
    private emitValueChange(): void {
        const ids = this.selected().map((x) => x.id);

        if (this.showChips) {
            // multi
            this._onChange(ids);
        } else {
            // single
            this._onChange(ids[0] ?? null);
        }
    }

    private applyIdsToUI(ids: string[], emit: boolean): void {
        const list = this._items();

        const matched = ids
            .map((id) => list.find((x) => x.id === id))
            .filter((x): x is AddableAutocompleteItem => !!x);

        // Update selection + visible text
        this.selected.set(this.showChips ? matched : matched.slice(0, 1));

        if (this.showChips) {
            // chips mode: keep input empty
            this.ctrl.setValue('', { emitEvent: false });
        } else {
            // single mode: show selected label in input
            const first = matched[0];
            this.ctrl.setValue(first?.label ?? '', { emitEvent: false });
        }

        if (emit) this.emitValueChange();
    }

    private markTouched(): void {
        this._onTouched();
    }
}
