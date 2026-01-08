// dental-service-dialog.component.ts
import { CommonModule, DatePipe } from '@angular/common';
import { Component, computed, inject, signal } from '@angular/core';
import { FormBuilder, ReactiveFormsModule, Validators } from '@angular/forms';
import { MAT_DIALOG_DATA, MatDialogModule, MatDialogRef } from '@angular/material/dialog';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatSelectModule } from '@angular/material/select';
import { MatCheckboxModule } from '@angular/material/checkbox';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatDividerModule } from '@angular/material/divider';
import { DentalServiceDialogData, DentalServiceDialogResult } from './dental-service-dialog.models';

import { toSignal } from '@angular/core/rxjs-interop';
import { startWith } from 'rxjs';

@Component({
  selector: 'app-dental-service-dialog',
  standalone: true,
  imports: [
    CommonModule,
    ReactiveFormsModule,

    MatDialogModule,
    MatFormFieldModule,
    MatInputModule,
    MatSelectModule,
    MatCheckboxModule,
    MatButtonModule,
    MatIconModule,
    MatDividerModule,

    DatePipe,
  ],
  templateUrl: './add-edit-dental-services.html',
  styleUrl: './add-edit-dental-services.scss',
})
export class DentalServiceDialogComponent {
  private fb = inject(FormBuilder);
  data = inject<DentalServiceDialogData>(MAT_DIALOG_DATA);

  readonly isEdit = computed(() => this.data.mode === 'edit');

  readonly form = this.fb.group({
    name: ['', [Validators.required, Validators.maxLength(120)]],
    type_id: [null as number | null, [Validators.required]],
    record_tooth: [false],
    active: [true],
  });

  /** Track current values as a signal */
  private readonly formValue = toSignal(
    this.form.valueChanges.pipe(startWith(this.form.getRawValue())),
    { initialValue: this.form.getRawValue() }
  );

  /** Snapshot of initial values (set after reset in ctor) */
  private readonly initialValue = signal(this.form.getRawValue());

  /** True when form differs from the initial snapshot */
  readonly hasChanges = computed(() => {
    // Stable enough here because keys are consistent and values are primitives.
    return JSON.stringify(this.formValue()) !== JSON.stringify(this.initialValue());
  });

  /** Used in template */
  readonly selectedTypeName = computed(() => {
    const typeId = this.form.controls.type_id.value;
    if (typeId == null) return '';
    return this.data.typeOptions?.find(t => t.id === typeId)?.name ?? '';
  });

  constructor(
    private dialogRef: MatDialogRef<DentalServiceDialogComponent, DentalServiceDialogResult>,
  ) {
    const s = this.data.service;

    const createDefaults = {
      name: '',
      type_id: this.data.typeOptions?.[0]?.id ?? null,
      record_tooth: false,
      active: true,
    };

    const editDefaults = s
      ? {
        name: s.name,
        type_id: s.type_id,
        record_tooth: s.record_tooth,
        active: s.active,
      }
      : createDefaults;

    this.form.reset(this.isEdit() ? editDefaults : createDefaults);

    // IMPORTANT: snapshot after reset, so "hasChanges" starts as false
    this.initialValue.set(this.form.getRawValue());
  }

  close() {
    this.dialogRef.close({ action: 'cancel' });
  }

  save() {
    if (this.form.invalid) {
      this.form.markAllAsTouched();
      return;
    }
    if (!this.hasChanges()) return;

    const raw = this.form.getRawValue();

    const payload = {
      name: raw.name!.trim(),
      type_id: raw.type_id!,
      record_tooth: !!raw.record_tooth,
      active: !!raw.active,
    };

    this.dialogRef.close({ action: 'save', payload });
  }
}
