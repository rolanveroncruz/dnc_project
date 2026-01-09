import {Component, inject, Inject} from '@angular/core';
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

type RoleDialogData = {
  name?:string;
  description?:string;
  active?:boolean;
  last_modified_by?:string;
  last_modified_on?:string | Date;
  // anything else
  [k:string]:any;
}

@Component({
  selector: 'app-add-edit-roles',
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
  private readonly original = {
    name: '',
    description: '',
  }
  form = this.fb.group({
    name: ['', [Validators.required]],
    description: ['', ],
  });

  constructor(
    @Inject(MAT_DIALOG_DATA) public data:RoleDialogData,
    private ref: MatDialogRef<AddEditRoles>,
  ){
      const initialName = data?.name ?? '';
      const initialDescription = data?.description ?? '';

      this.original = {
        name: initialName,
        description: initialDescription,
      }
      this.form.setValue({
        name: initialName,
        description: initialDescription,
      });

      // Track if editable fields changed vs original snapshot
    this.form.valueChanges.subscribe((v) =>{
      this.hasChanges =
        (v.name ?? '') !== (this.original.name ?? '') ||
        (v.description ?? '') !== (this.original.description ?? '');
    });
  }

  close() {
    this.ref.close();
  }

  save() {
    if (this.form.invalid) return;

    const updated = {
      ...this.data,
      ...this.form.getRawValue(),
    };

    this.ref.close(updated);
  }
}
