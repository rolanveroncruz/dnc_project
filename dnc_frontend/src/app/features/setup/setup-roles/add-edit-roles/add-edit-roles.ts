import {Component, Inject} from '@angular/core';
import {JsonPipe} from '@angular/common';
import {MatButton} from '@angular/material/button';
import {
  MAT_DIALOG_DATA,
  MatDialogActions,
  MatDialogContent,
  MatDialogRef,
  MatDialogTitle
} from '@angular/material/dialog';

@Component({
  selector: 'app-add-edit-roles',
  imports: [
    JsonPipe,
    MatButton,
    MatDialogActions,
    MatDialogContent,
    MatDialogTitle
  ],
  templateUrl: './add-edit-roles.html',
  styleUrl: './add-edit-roles.scss',
})
export class AddEditRoles {
  constructor(@Inject(MAT_DIALOG_DATA) public data:any,
              private ref: MatDialogRef<AddEditRoles>){}

  close() {
    this.ref.close();
  }

  save() {
    this.ref.close(this.data);
  }


}
