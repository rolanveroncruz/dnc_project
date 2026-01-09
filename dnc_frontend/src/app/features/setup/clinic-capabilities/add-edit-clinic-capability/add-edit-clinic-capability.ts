import { CommonModule, DatePipe } from '@angular/common';
import { Component, Inject, computed, effect, inject, signal } from '@angular/core';
import { FormBuilder, ReactiveFormsModule, Validators } from '@angular/forms';
import {
  MAT_DIALOG_DATA,
  MatDialogActions,
  MatDialogContent,
  MatDialogRef,
  MatDialogTitle,
} from '@angular/material/dialog';
import { MatButtonModule } from '@angular/material/button';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';

export interface ClinicCapability {
  id: number;
  name: string;
  last_modified_by: string;
  last_modified_on: Date;
}

type DialogData = Partial<ClinicCapability> & Record<string, any>;

@Component({
  selector: 'app-add-edit-clinic-capability',
  standalone: true,
  imports: [
    CommonModule,
    DatePipe,
    ReactiveFormsModule,

    MatDialogTitle,
    MatDialogContent,
    MatDialogActions,

    MatFormFieldModule,
    MatInputModule,
    MatButtonModule,
  ],
  templateUrl: './add-edit-clinic-capability.html',
  styleUrl: './add-edit-clinic-capability.scss',
})
export class AddEditClinicCapability {
  private fb = inject(FormBuilder);
  private ref = inject(MatDialogRef<AddEditClinicCapability>);
  public data = inject(MAT_DIALOG_DATA) as DialogData;

  /** Edit vs New */
  readonly isEdit = !!this.data?.id;

  /** Form (editable fields only) */
  readonly form = this.fb.nonNullable.group({
    name: this.fb.nonNullable.control(this.data?.name ?? '', {
      validators: [Validators.required, Validators.maxLength(120)],
    }),
  });

  /**
   * Track initial form snapshot so Save enables only after edits.
   * (We compare current value to initial value.)
   */
  private readonly initialSnapshot = signal(this.form.getRawValue());

  /** Exposed boolean for template (@if(hasChanges) + [disabled]) */
  readonly hasChanges = computed(() => {
    const a = this.initialSnapshot();
    const b = this.form.getRawValue();
    return !this.shallowEqual(a, b);
  });

  constructor(@Inject(MAT_DIALOG_DATA) _data: any) {
    // If the dialog ever receives a different "data" object instance,
    // reset form + baseline snapshot. (Also handy if you later reuse the component.)
    effect(() => {
      // no reactive deps here; left intentionally simple
    });
  }

  close() {
    this.ref.close();
  }

  save() {
    // Touch everything so errors show if user clicked Save via keyboard automation etc.
    this.form.markAllAsTouched();
    if (this.form.invalid || !this.hasChanges()) return;

    const v = this.form.getRawValue();

    // Return merged object: keep readonly fields from incoming data, update editable fields from form.
    const result: DialogData = {
      ...this.data,
      name: v.name.trim(),
    };

    this.ref.close(result);
  }

  // -------- helpers --------

  private shallowEqual(a: Record<string, any>, b: Record<string, any>) {
    const ak = Object.keys(a);
    const bk = Object.keys(b);
    if (ak.length !== bk.length) return false;
    for (const k of ak) {
      if (a[k] !== b[k]) return false;
    }
    return true;
  }
}
