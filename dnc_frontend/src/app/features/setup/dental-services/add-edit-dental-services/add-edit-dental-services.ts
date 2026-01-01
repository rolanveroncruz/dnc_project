import { Component, Inject } from '@angular/core';
import { MAT_DIALOG_DATA, MatDialogModule, MatDialogRef } from '@angular/material/dialog';
import {MatButtonModule} from '@angular/material/button';
import {CommonModule} from '@angular/common';

@Component({
  selector: 'app-add-edit-dental-services',
  imports: [
    CommonModule,
    MatDialogModule,
    MatButtonModule
  ],
  templateUrl: './add-edit-dental-services.html',
  styleUrl: './add-edit-dental-services.scss',
})
export class AddEditDentalServices {
  constructor(@Inject(MAT_DIALOG_DATA) public data:any,
              private ref: MatDialogRef<AddEditDentalServices>){}

  close() {
    this.ref.close();
  }

  save() {
    this.ref.close(this.data);
  }

}
