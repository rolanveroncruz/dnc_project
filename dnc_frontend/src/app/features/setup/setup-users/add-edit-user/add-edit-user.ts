import { CommonModule, DatePipe } from '@angular/common';
import { Component, inject } from '@angular/core';
import { FormBuilder, ReactiveFormsModule, Validators } from '@angular/forms';
import { MAT_DIALOG_DATA, MatDialogModule, MatDialogRef } from '@angular/material/dialog';

import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatSelectModule } from '@angular/material/select';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { AbstractControl, ValidationErrors, ValidatorFn } from '@angular/forms';
import { User } from '../../../../api_services/user-service';

export interface RoleOption {
  id: number;
  name: string;
}

export type UserDialogMode = 'create' | 'edit';

export type UserDialogData = {
  mode: UserDialogMode;
  user?: Partial<User>;     // present for edit; optional for create
  roles: RoleOption[];      // dropdown options
};

export type UserDialogResult =
  | { action: 'cancel' }
  | { action: 'save'; payload: { name: string; email: string; role_id: number; password?: string } };

type UserFormValue = {
  name: string;
  email: string;
  role_id: number | null;
  password: string;
};

@Component({
  selector: 'app-add-edit-user-dialog',
  standalone: true,
  imports: [
    CommonModule,
    DatePipe,
    ReactiveFormsModule,

    MatDialogModule,
    MatFormFieldModule,
    MatInputModule,
    MatSelectModule,
    MatButtonModule,
    MatIconModule,
  ],
  templateUrl: './add-edit-user.html',
  styleUrls: ['./add-edit-user.scss'],
})
export class AddEditUserDialogComponent {
  hidePassword = true;
  togglePasswordVisibility(){
    this.hidePassword = !this.hidePassword;
  }

  private fb = inject(FormBuilder);
  data = inject<UserDialogData>(MAT_DIALOG_DATA);

  readonly isEdit = this.data.mode === 'edit';

  form = this.fb.group({
    name: this.fb.nonNullable.control('', [Validators.required, Validators.maxLength(120)]),
    email: this.fb.nonNullable.control('', [Validators.required, Validators.email, Validators.maxLength(160)]),
    role_id: this.fb.control<number | null>(null, [Validators.required]),
    password: this.fb.nonNullable.control('', []),
  });

  private initialValue!: UserFormValue;

  constructor(
    private dialogRef: MatDialogRef<AddEditUserDialogComponent, UserDialogResult>,
  ) {
    const pwdCtrl = this.form.controls.password;
    if (this.isEdit){
      pwdCtrl.setValidators([optionalMinLength(8)]);
    }else{
      pwdCtrl.setValidators([Validators.required, Validators.minLength(8)]);
    }
    pwdCtrl.updateValueAndValidity({emitEvent: false});

    // Seed form
    if (this.data.user) {
      this.form.patchValue({
        name: this.data.user.name ?? '',
        email: this.data.user.email ?? '',
        role_id: (this.data.user.role_id as number | null) ?? null,
      });
    }

    // Snapshot AFTER seeding
    this.initialValue = this.snapshot(this.form.getRawValue() as UserFormValue);
  }

  /** True only when current values differ from the initial snapshot (trim-aware for strings). */
  get hasChanges(): boolean {
    const cur = this.snapshot(this.form.getRawValue() as UserFormValue);

    const passwordChanged = this.isEdit? cur.password.length>0 : cur.password!==this.initialValue.password;

    return (
      cur.name !== this.initialValue.name ||
      cur.email !== this.initialValue.email ||
      cur.role_id !== this.initialValue.role_id ||
        passwordChanged
    );
  }

  /** Friendly role name based on selected role_id */
  get selectedRoleName(): string {
    const rid = this.form.controls.role_id.value;
    if (rid == null) return '';
    return this.data.roles.find(r => r.id === rid)?.name ?? '';
  }

  private snapshot(v: UserFormValue): UserFormValue {
    return {
      name: (v.name ?? '').trim(),
      email: (v.email ?? '').trim(),
      role_id: v.role_id ?? null,
      password: v.password ?? '',
    };
  }

  close(): void {
    this.dialogRef.close({ action: 'cancel' });
  }

  save(): void {
    if (this.form.invalid) {
      this.form.markAllAsTouched();
      return;
    }

    const v = this.snapshot(this.form.getRawValue() as UserFormValue);
    console.log("in save(), v:",v)

    this.dialogRef.close({
      action: 'save',
      payload: {
        name: v.name,
        email: v.email,
        role_id: v.role_id as number, // required validator ensures non-null
        ...( v.password ? { password: v.password} :{}),
      },
    });
  }

  onEnter(): void {
    this.save();
  }
}

function optionalMinLength(min: number): ValidatorFn {
  return (control: AbstractControl): ValidationErrors | null => {
    const v = (control.value ?? '') as string;
    if (!v) return null; // empty is allowed
    return Validators.minLength(min)(control);
  };
}

