import { CommonModule } from '@angular/common';
import {Component,  computed, inject} from '@angular/core';
import { FormBuilder, ReactiveFormsModule, Validators } from '@angular/forms';
import { MAT_DIALOG_DATA, MatDialogModule, MatDialogRef } from '@angular/material/dialog';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatSelectModule } from '@angular/material/select';
import { MatInputModule } from '@angular/material/input';
import { MatCheckboxModule } from '@angular/material/checkbox';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatDividerModule } from '@angular/material/divider';
import { MatTooltipModule } from '@angular/material/tooltip';
import {ModifiedRolePermission} from '../../../../api_services/roles-and-permissions-service';

export type PermissionAction = 'create' | 'read' | 'update';

export interface RoleOption {
  id: number;
  name: string;
}

export interface DataObjectOption {
  id: number;
  name: string; // e.g. "role", "dental_service", "clinic_capability"
  label?: string; // optional friendly label
}

// export interface RolePermissionRow {
//   id: number;
//   role_id: number;
//   role_name: string;
//
//   data_object_id: number;
//   data_object_name: string;
//
//   // however, you represent actions today:
//   can_create: boolean;
//   can_read: boolean;
//   can_update: boolean;
//
//   last_modified_by: string;
//   last_modified_on: string; // ISO string or already formatted string
// }

export type AddEditRolePermissionMode = 'add' | 'edit';

export interface AddEditRolePermissionsDialogData {
  mode: AddEditRolePermissionMode;

  // for edit:
  row?: ModifiedRolePermission;

  // dropdown sources:
  roles: RoleOption[];
  objects: DataObjectOption[];
}

export interface AddEditRolePermissionsDialogResult {
  mode: AddEditRolePermissionMode;

  // if edit, include id
  id?: number;

  role_id: number;
  data_object_id: number;

  actions: PermissionAction[]; // derived from checkboxes
}

@Component({
  selector: 'app-add-edit-role-permissions-dialog',
  standalone: true,
  imports: [
    CommonModule,
    ReactiveFormsModule,

    MatDialogModule,
    MatFormFieldModule,
    MatSelectModule,
    MatInputModule,
    MatCheckboxModule,
    MatButtonModule,
    MatIconModule,
    MatDividerModule,
    MatTooltipModule,
  ],
  templateUrl: './add-edit-role-permissions.dialog.html',
  styleUrls: ['./add-edit-role-permissions.dialog.scss'],
})
export class AddEditRolePermissionsDialogComponent {
  private fb = new FormBuilder();
  private dialogRef = inject(MatDialogRef<AddEditRolePermissionsDialogComponent, AddEditRolePermissionsDialogResult>);
  readonly data = inject<AddEditRolePermissionsDialogData>(MAT_DIALOG_DATA);
  mode: AddEditRolePermissionMode = this.data.mode;



  // Main form
  form = this.fb.group({
    role_id: this.fb.control<number | null>(null, { validators: [Validators.required] }),
    data_object_id: this.fb.control<number | null>(null, { validators: [Validators.required] }),

    actions: this.fb.group({
      create: this.fb.control(false),
      read: this.fb.control(true),   // common default is read=true; adjust if you want
      update: this.fb.control(false),
    }),

    // audit (readonly)
    id: this.fb.control({ value: '', disabled: true }),
    last_modified_by: this.fb.control({ value: '', disabled: true }),
    last_modified_on: this.fb.control({ value: '', disabled: true }),
  });

  title = computed(() => (this.mode === 'add' ? 'Add Role Permission' : 'Edit Role Permission'));
  subtitle = computed(() =>
    this.mode === 'add'
      ? 'Define what a role can do for a specific data object.'
      : 'Update the permissions for this role and object.'
  );

  constructor(
  ) {
    console.log("In AERPDC constructor. Data is", this.data)
    console.log("In AERPDC constructor. Row:", this.data.row)
    if (this.mode === 'edit' && this.data.row) {
      this.patchFromRow(this.data.row);
    } else {
      // defaults for add can go here
      this.form.controls.actions.patchValue({ read: true });
    }
  }

  private patchFromRow(row: ModifiedRolePermission) {
    this.form.patchValue({
      role_id: row.role_id,
      data_object_id: row.object_id,
      actions: { create: row.actions.includes('create'),
        read: row.actions.includes('read'),
        update: row.actions.includes('update'),
      },
    });

    this.form.controls.id.setValue(String(row.id));
    this.form.controls.last_modified_by.setValue(row.last_modified_by ?? '');
    this.form.controls.last_modified_on.setValue(row.last_modified_on.toLocaleString() ?? '');

    console.log('row.data_object_id', this.data.row?.object_id, typeof this.data.row?.object_id);
    console.log('data_objects[0].id', this.data.objects?.[0]?.id, typeof this.data.objects?.[0]?.id);
  }

  cancel() {
    this.dialogRef.close();
  }

  save() {
    this.form.markAllAsTouched();
    if (this.form.invalid) return;

    const v = this.form.getRawValue();

    const actions: PermissionAction[] = [];
    if (v.actions?.create) actions.push('create');
    if (v.actions?.read) actions.push('read');
    if (v.actions?.update) actions.push('update');

    // You can decide to require at least one action:
    if (actions.length === 0) return;

    this.dialogRef.close({
      mode: this.mode,
      id: this.data.row?.id,
      role_id: v.role_id!,
      data_object_id: v.data_object_id!,
      actions,
    });
  }

  // convenience for template
  get roleCtrl() { return this.form.controls.role_id; }
  get objectCtrl() { return this.form.controls.data_object_id; }
  get actionsGroup() { return this.form.controls.actions; }
}
