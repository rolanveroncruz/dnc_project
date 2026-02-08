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
import {toSignal} from '@angular/core/rxjs-interop';
import { map, startWith } from 'rxjs';

export interface ClinicCapability {
  id: number;
  name: string;
  last_modified_by: string;
  last_modified_on: Date;
}

type DialogData = Partial<ClinicCapability> & Record<string, any>;

export type ClinicCapabilityDialogMode = 'create' | 'edit';

export type ClinicCapabilityDialogResult =
  | { action: 'cancel' }
  | {
  action: 'save';
  mode: ClinicCapabilityDialogMode;
  payload: { id?: number; name: string };
};
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
  private ref = inject(MatDialogRef<AddEditClinicCapability, ClinicCapabilityDialogResult>);
  public data = inject(MAT_DIALOG_DATA) as DialogData;
  private readonly capability: Partial<ClinicCapability> = this.data ?? {};
  readonly mode: ClinicCapabilityDialogMode = this.data?.id ? 'edit' : 'create';

  /** Edit vs New */
  readonly isEdit = this.mode === 'edit';


  /** Form (editable fields only) */
  readonly form = this.fb.nonNullable.group({
    name: this.fb.nonNullable.control(this.data?.name ?? '', {
      validators: [Validators.required, Validators.maxLength(120)],
    }),
  });

  private readonly formValue = toSignal(
    this.form.valueChanges.pipe(startWith(this.form.getRawValue())),
    { initialValue: this.form.getRawValue() }
  );
  private readonly isValid = toSignal(
    this.form.statusChanges.pipe(
      startWith(this.form.status),
      map((s)=> s === 'VALID')
    ),
    { initialValue: this.form.valid }
  );

  /**
   * Track initial form snapshot so Save enables only after edits.
   * (We compare current value to initial value.)
   */
  private readonly initialSnapshot = signal(this.normalize(this.form.getRawValue()));

  private normalize(v:{name: string}) {
    return {name:(v.name ?? '').trim()};
  }



  /** Exposed boolean for template (@if(hasChanges) + [disabled]) */
  readonly hasChanges = computed(() => {
    const a = this.initialSnapshot();
    const b = this.normalize(this.formValue()! as {name: string});
    return !this.shallowEqual(a, b);
  });

  readonly saveDisabled = computed(()=> !this.isValid() || !this.hasChanges());

  constructor() {
    // If the dialog ever receives a different "data" object instance,
    // reset form + baseline snapshot. (Also handy if you later reuse the component.)
    effect(() => {
      // no reactive deps here; left intentionally simple
    });
  }

  close() {
    console.log("in close(), canceling")
    this.ref.close({action: 'cancel'});
  }

  save() {
    // Touch everything so errors show if user clicked Save via keyboard automation etc.
    this.form.markAllAsTouched();
    if (this.form.invalid || !this.hasChanges()) return;

    const v = this.normalize(this.form.getRawValue());

    // Return merged object: keep readonly fields from incoming data, update editable fields from form.
    const result: ClinicCapabilityDialogResult = {
      action: 'save',
      mode: this.mode,
      payload: {
        ...(this.capability.id ? {id: this.capability.id} : {}),
        name: v.name,
      },
    };
    console.log("in save(), closing with result:",result)

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
