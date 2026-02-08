import { Component, inject, signal, computed } from '@angular/core';
import { FormBuilder, ReactiveFormsModule, Validators } from '@angular/forms';
import { map, startWith } from 'rxjs';
import { MatDialogModule, MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { MatButtonModule } from '@angular/material/button';
import { MatListModule } from '@angular/material/list';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatIconModule } from '@angular/material/icon';

export type AddClinicOrDentistMode = 'dentist' | 'clinic';

export interface AddClinicOrDentistOption {
    id: number;
    name: string;
}

export interface AddClinicOrDentistDialogData {
    mode: AddClinicOrDentistMode;               // 'dentist' or 'clinic'
    options: AddClinicOrDentistOption[];        // list to pick from
    preselectedId?: number | null;              // optional
    initialPosition?: string | null;            // optional
    initialSchedule?: string | null;            // optional
    title?: string;                             // optional custom title
}

export interface AddClinicOrDentistDialogResult {
    mode: AddClinicOrDentistMode;
    selected: AddClinicOrDentistOption;
    position: string;
    schedule: string;
}

@Component({
    selector: 'app-add-clinic-or-dentist-dialog',
    standalone: true,
    imports: [
        ReactiveFormsModule,
        MatDialogModule,
        MatButtonModule,
        MatListModule,
        MatFormFieldModule,
        MatInputModule,
        MatIconModule,
    ],
    templateUrl: './add-clinic-or-dentist-dialog-component.html',
    styleUrl: './add-clinic-or-dentist-dialog-component.scss',
})
export class AddClinicOrDentistDialogComponent {
    private readonly dialogRef =
        inject(MatDialogRef<AddClinicOrDentistDialogComponent, AddClinicOrDentistDialogResult | null>);
    readonly data = inject<AddClinicOrDentistDialogData>(MAT_DIALOG_DATA);
    private readonly fb = inject(FormBuilder);

    // Keep selected option id in a signal (because selection list is not a form control)
    readonly selectedId = signal<number | null>(this.data.preselectedId ?? null);

    readonly form = this.fb.nonNullable.group({
        position: [this.data.initialPosition ?? '', []],
        schedule: [this.data.initialSchedule ?? '', []],
    });
    readonly filterCtrl = this.fb.nonNullable.control('');
    readonly filterText = signal('');
    readonly filteredOptions = computed(() => {
        const q = this.filterText().trim().toLowerCase();
        if (!q) return this.data.options;
        return this.data.options.filter(o => o.name.toLowerCase().includes(q));
    });


    readonly title = computed(() => {
        if (this.data.title?.trim()) return this.data.title.trim();
        return this.data.mode === 'dentist' ? 'Add Dentist' : 'Add Clinic';
    });

    readonly listLabel = computed(() =>
        this.data.mode === 'dentist' ? 'Select a dentist' : 'Select a clinic'
    );

    readonly selectedOption = computed(() => {
        const id = this.selectedId();
        if (id == null) return null;
        return this.data.options.find(o => o.id === id) ?? null;
    });

    readonly canSave = computed(() => !!this.selectedOption());

    constructor() {
        this.filterCtrl.valueChanges
            .pipe(startWith(this.filterCtrl.value))
            .subscribe(v => this.filterText.set(v ?? ''));
    }
    clearFilter() {
        this.filterCtrl.setValue('');
    }


    onSelectionChange(id: number) {
        this.selectedId.set(id);
    }

    onCancel() {
        this.dialogRef.close(null);
    }

    onSave() {
        const selected = this.selectedOption();
        if (!selected) return;

        const position = this.form.controls.position.value.trim();
        const schedule = this.form.controls.schedule.value.trim();

        this.dialogRef.close({
            mode: this.data.mode,
            selected,
            position,
            schedule,
        });
    }
}
