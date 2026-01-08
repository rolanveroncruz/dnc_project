import { CommonModule } from '@angular/common';
import { Component, computed, inject, DestroyRef } from '@angular/core';
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
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';
import { startWith } from 'rxjs';
import { ModifiedRolePermission } from '../../../../api_services/roles-and-permissions-service';

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

type EditableSnapshot = {
  role_id: number | null;
  data_object_id: number | null;
  actions: { create: boolean; read: boolean; update: boolean };
};

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
  private fb = inject(FormBuilder);
  private destroyRef = inject(DestroyRef);
  private dialogRef =
    inject(MatDialogRef<AddEditRolePermissionsDialogComponent, AddEditRolePermissionsDialogResult>);
  readonly data = inject<AddEditRolePermissionsDialogData>(MAT_DIALOG_DATA);

  mode: AddEditRolePermissionMode = this.data.mode;

  // --- Editable form only (audit is displayed from data.row on the right) ---
  form = this.fb.group({
    role_id: this.fb.control<number | null>(null, { validators: [Validators.required] }),
    data_object_id: this.fb.control<number | null>(null, { validators: [Validators.required] }),
    actions: this.fb.group({
      create: this.fb.control(false),
      read: this.fb.control(true),
      update: this.fb.control(false),
    }),
  });

  title = computed(() => (this.mode === 'add' ? 'Add Role Permission' : 'Edit Role Permission'));
  subtitle = computed(() =>
    this.mode === 'add'
      ? 'Define what a role can do for a specific data object.'
      : 'Update the permissions for this role and object.'
  );

  // --- Derived UI state ---
  hasChanges = false;     // net-change vs baseline snapshot
  hasAnyAction = true;    // at least one checkbox

  private baseline: EditableSnapshot = this.snapshot();

  constructor() {
    if (this.mode === 'edit' && this.data.row) {
      this.patchFromRow(this.data.row);
    } else {
      // defaults for add
      this.form.controls.actions.patchValue({ read: true }, { emitEvent: false });
      this.commitBaseline();
    }

    // keep flags updated, including on init
    this.form.valueChanges
      .pipe(startWith(this.form.getRawValue()), takeUntilDestroyed(this.destroyRef))
      .subscribe(() => this.recomputeFlags());
  }

  private patchFromRow(row: ModifiedRolePermission) {
    this.form.patchValue(
      {
        role_id: row.role_id,
        data_object_id: row.object_id,
        actions: {
          create: row.actions.includes('create'),
          read: row.actions.includes('read'),
          update: row.actions.includes('update'),
        },
      },
      { emitEvent: false }
    );

    this.commitBaseline();
  }

  private snapshot(): EditableSnapshot {
    const v = this.form.getRawValue();
    return {
      role_id: v.role_id ?? null,
      data_object_id: v.data_object_id ?? null,
      actions: {
        create: !!v.actions?.create,
        read: !!v.actions?.read,
        update: !!v.actions?.update,
      },
    };
  }

  private commitBaseline() {
    this.baseline = this.snapshot();
    this.form.markAsPristine();
    this.form.markAsUntouched();
    this.recomputeFlags();
  }

  private recomputeFlags() {
    const cur = this.snapshot();
    this.hasAnyAction = !!(cur.actions.create || cur.actions.read || cur.actions.update);
    this.hasChanges = !this.deepEqual(cur, this.baseline);
  }

  private deepEqual(a: unknown, b: unknown) {
    // Stable enough here because keys are consistent + shallow structure
    return JSON.stringify(a) === JSON.stringify(b);
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

  get selectedRoleName(): string {
    const id = this.roleCtrl.value;
    return this.data.roles.find(r => r.id === id)?.name ?? '—';
  }

  get selectedObjectLabel(): string {
    const id = this.objectCtrl.value;
    const o = this.data.objects.find(x => x.id === id);
    return (o?.label ?? o?.name) ?? '—';
  }
}
