import { CommonModule } from '@angular/common';
import {
    Component,
    DestroyRef,
    ElementRef,
    Input,
    Output,
    EventEmitter,
    ViewChild,
    computed,
    inject,
    signal,
} from '@angular/core';
import { FormControl, ReactiveFormsModule } from '@angular/forms';
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';

import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatAutocompleteModule, MatAutocompleteSelectedEvent } from '@angular/material/autocomplete';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';
import { MatChipsModule } from '@angular/material/chips';

import { map, startWith } from 'rxjs/operators';

export type AddableAutocompleteItem = {
    id: string;
    label: string;
};

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
})
export class AddableAutocompleteComponent {
    private readonly destroyRef = inject(DestroyRef);

    // ---------- Inputs ----------
    @Input() label = 'Select or add';
    @Input() placeholder = 'Type to search…';
    @Input() hint = '';
    @Input() disabled = false;

    /**
     * The source list (shown in the dropdown).
     * You can pass your own list from parent.
     */
    @Input({ required: true }) set items(value: AddableAutocompleteItem[]) {
        this._items.set(value ?? []);
    }

    /**
     * Optional: if you want to show selected items as chips (multi-select feel).
     * If false, it behaves like a single select/add field.
     */
    @Input() showChips = true;

    /**
     * If true: selecting an option also adds it to "selected" (chips).
     * If false: selecting just fills the input.
     */
    @Input() selectAddsToChips = true;

    // ---------- Outputs ----------
    /** Fires when the user creates a new item. */
    @Output() itemCreated = new EventEmitter<AddableAutocompleteItem>();

    /** Fires when the user "selects" an existing option (from dropdown). */
    @Output() itemSelected = new EventEmitter<AddableAutocompleteItem>();

    /** Fires when the selected chips list changes (add/remove). */
    @Output() selectedChange = new EventEmitter<AddableAutocompleteItem[]>();

    @ViewChild('textInput', { static: true }) textInput!: ElementRef<HTMLInputElement>;

    // ---------- Internal State ----------
    readonly ctrl = new FormControl<string>('', { nonNullable: true });

    private readonly _items = signal<AddableAutocompleteItem[]>([]);

    readonly selected = signal<AddableAutocompleteItem[]>([]);

    // Filtered options for the autocomplete dropdown
    readonly filteredOptions = signal<AddableAutocompleteItem[]>([]);

    constructor() {
        // keep filteredOptions in sync with ctrl.value
        this.ctrl.valueChanges
            .pipe(
                startWith(this.ctrl.value),
                map((v) => (v ?? '').trim()),
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
                        .filter((x) => x.label.toLowerCase().includes(q))
                        .slice(0, 50),
                );
            });
    }

    // ---------- UI helpers ----------

    isBusyAddable = computed(() => {
        const text = this.ctrl.value.trim();
        if (!text) return false;
        // add button should show if the item doesn't already exist (case-insensitive label match)
        const exists = this._items().some((x) => x.label.toLowerCase() === text.toLowerCase());
        return !exists;
    });

    // ---------- Actions ----------
    onOptionSelected(ev: MatAutocompleteSelectedEvent) {
        const picked = ev.option.value as AddableAutocompleteItem;

        this.itemSelected.emit(picked);

        if (this.selectAddsToChips) {
            this.addToSelected(picked);
            this.ctrl.setValue('');
            this.focusInput();
        } else {
            this.ctrl.setValue(picked.label);
            this.focusInput();
        }
    }

    addTypedValue() {
        const label = this.ctrl.value.trim();
        if (!label) return;

        const existing = this._items().find((x) => x.label.toLowerCase() === label.toLowerCase());
        if (existing) {
            // treat as selecting existing
            this.itemSelected.emit(existing);
            if (this.selectAddsToChips) {
                this.addToSelected(existing);
                this.ctrl.setValue('');
                this.focusInput();
            }
            return;
        }

        // Create the new item (client-side id; parent can then replace with real id if needed)
        const created: AddableAutocompleteItem = {
            id: crypto.randomUUID(),
            label,
        };

        // update local list
        this._items.set([created, ...this._items()]);

        // emit creation so parent can persist it
        this.itemCreated.emit(created);

        // also select it (chips)
        if (this.selectAddsToChips) {
            this.addToSelected(created);
            this.ctrl.setValue('');
        } else {
            this.ctrl.setValue(created.label);
        }

        this.focusInput();
    }

    removeChip(item: AddableAutocompleteItem) {
        const next = this.selected().filter((x) => x.id !== item.id);
        this.selected.set(next);
        this.selectedChange.emit(next);
    }

    clearInput() {
        this.ctrl.setValue('');
        this.focusInput();
    }

    onEnterKey() {
        // If the typed value is addable, add it; else do nothing special.
        if (this.isBusyAddable()) {
            this.addTypedValue();
        }
    }

    private addToSelected(item: AddableAutocompleteItem) {
        // prevent duplicates
        if (this.selected().some((x) => x.id === item.id)) return;
        const next = [...this.selected(), item];
        this.selected.set(next);
        this.selectedChange.emit(next);
    }

    private focusInput() {
        queueMicrotask(() => this.textInput?.nativeElement?.focus());
    }

    displayWith(x: AddableAutocompleteItem | null): string {
        return x?.label ?? '';
    }

}
