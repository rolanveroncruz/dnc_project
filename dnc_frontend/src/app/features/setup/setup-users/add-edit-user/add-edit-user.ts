import { CommonModule, DatePipe } from '@angular/common';
import {Component,  computed, inject} from '@angular/core';
import { FormBuilder, ReactiveFormsModule, Validators } from '@angular/forms';
import { MAT_DIALOG_DATA, MatDialogModule, MatDialogRef } from '@angular/material/dialog';

import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatSelectModule } from '@angular/material/select';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatDividerModule } from '@angular/material/divider';
import { MatTooltipModule } from '@angular/material/tooltip';

import {User} from '../../../../api_services/user-service';

export interface RoleOption {
  id: number;
  name: string;
}

export type UserDialogMode = 'create' | 'edit';

export type UserDialogData = {
  mode: UserDialogMode;
  user?: Partial<User>;         // present for edit; optional for create
  roles: RoleOption[];          // dropdown options
};

export type UserDialogResult =
  | { action: 'cancel' }
  | { action: 'save'; payload: { name: string; email: string; role_id: number } };

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
    MatDividerModule,
    MatTooltipModule,
  ],
  templateUrl: './add-edit-user.html',
  styleUrls: ['./add-edit-user.scss'],
})
export class AddEditUserDialogComponent {
  private fb = inject(FormBuilder);
  data = inject<UserDialogData>(MAT_DIALOG_DATA);
  constructor(
    private dialogRef: MatDialogRef<AddEditUserDialogComponent, UserDialogResult>,
  ) {
    if (this.data.user) {
      this.form.patchValue({
        name: this.data.user.name ?? '',
        email: this.data.user.email ?? '',
        role_id: this.data.user.role_id ?? null,
      });
    }
  }

  readonly isEdit = this.data.mode === 'edit';

  form = this.fb.group({
    name: this.fb.nonNullable.control('', [Validators.required, Validators.maxLength(120)]),
    email: this.fb.nonNullable.control('', [Validators.required, Validators.email, Validators.maxLength(160)]),
    role_id: this.fb.control<number | null>(null, [Validators.required]),
  });

  selectedRoleName = computed(() => {
    const rid = this.form.controls.role_id.value;
    if (rid == null) return '';
    return this.data.roles.find(r => r.id === rid)?.name ?? '';
  });

  close(): void {
    this.dialogRef.close({ action: 'cancel' });
  }

  save(): void {
    if (this.form.invalid) {
      this.form.markAllAsTouched();
      return;
    }
    const v = this.form.getRawValue();
    // role_id is validated required, but still typed nullable -> assert after validation
    this.dialogRef.close({
      action: 'save',
      payload: {
        name: v.name.trim(),
        email: v.email.trim(),
        role_id: v.role_id as number,
      },
    });
  }

  // Optional: if you want Enter key to save
  onEnter(): void {
    this.save();
  }
}
