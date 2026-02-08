import {Component, inject, Inject, } from '@angular/core';
import { DatePipe} from '@angular/common';
import {ReactiveFormsModule, FormBuilder,  Validators} from '@angular/forms';
import {MatButton} from '@angular/material/button';
import {MatError, MatFormField, MatHint, MatLabel} from '@angular/material/form-field';
import {MatInput} from '@angular/material/input';
import {
  MAT_DIALOG_DATA,
  MatDialogActions,
  MatDialogContent,
  MatDialogRef,
  MatDialogTitle
} from '@angular/material/dialog';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';

export type RoleDialogMode = 'add' | 'edit';

type RoleDialogData = {
  mode: RoleDialogMode;
  row?: {
    id: number;
    name?:string;
    description?:string;
    active?:boolean;
    last_modified_by?:string;
    last_modified_on?:string | Date;
    // anything else
     [k:string]:any;
  }
  [k:string]:any;
}

@Component({
  selector: 'app-add-edit-roles',
  standalone: true,
  imports: [
    DatePipe,
    ReactiveFormsModule,

    MatButton,
    MatFormField,
    MatInput,
    MatLabel,
    MatDialogActions,
    MatDialogContent,
    MatDialogTitle,
    MatError,
    MatHint
  ],
  templateUrl: './add-edit-roles.html',
  styleUrl: './add-edit-roles.scss',
})
export class AddEditRoles {
  private fb = inject(FormBuilder);

  hasChanges = false;
  // snapshot of the data passed in from the dialog
  private original = {
    name: '',
    description: '',
  }
  form = this.fb.group({
    name: ['', [Validators.required]],
    description: ['',Validators.maxLength(500) ],
  });

  constructor(
    @Inject(MAT_DIALOG_DATA) public data:RoleDialogData,
    private ref: MatDialogRef<AddEditRoles>,
  ){
     console.log("In constructor:",this.data);
      const initialName = this.data?.row?.name ?? '';
      const initialDescription = this.data?.row?.description ?? '';

      this.original = {
        name: initialName,
        description: initialDescription,
      }
      this.form.patchValue({
        name: initialName,
        description: initialDescription,
      });

      // Track if editable fields changed vs original snapshot
    this.form.valueChanges
      .pipe(takeUntilDestroyed())
      .subscribe((v) =>{
      this.hasChanges =
        (v.name ?? '') !== (this.original.name ?? '') ||
        (v.description ?? '') !== (this.original.description ?? '');
    });
    this.hasChanges = false;
  }

  title(): string {
    return this.data.mode === 'edit' ? 'Edit Role' : 'New Role';
  }

  subtitle(): string {
    return this.data.mode === 'edit' ?
      'Update role name and description'
      : 'Create a new role';
  }

  isEdit(): boolean {
    return this.data.mode === 'edit';
  }

  close() {
    this.ref.close();
  }

  save() {
    if (this.form.invalid) return;

    const payload = {
      mode: this.data.mode,
      id: this.data.row?.id,
      ...this.data?.row,
      ...this.form.getRawValue(),
    };

    this.ref.close(payload);
  }
}
