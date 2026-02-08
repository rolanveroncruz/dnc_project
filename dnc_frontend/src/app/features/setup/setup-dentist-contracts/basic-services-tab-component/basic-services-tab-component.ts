import { Component,Input } from '@angular/core';
import {FormGroup, ReactiveFormsModule} from '@angular/forms';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatInputModule} from '@angular/material/input';

export interface DentalService{
  id: number;
  name: string;
}

@Component({
  selector: 'app-basic-services-tab-component',
  imports: [
    ReactiveFormsModule,
    MatFormFieldModule,
    MatInputModule
  ],
  templateUrl: './basic-services-tab-component.html',
  styleUrl: './basic-services-tab-component.scss',
  standalone: true
})
export class BasicServicesTabComponent {
  @Input({required: true}) services: DentalService[] =[];
  @Input({required: true}) ratesGroup!: FormGroup;
  @Input() disabled = false;

  controlName(serviceId:number){
    return String(serviceId);
  }

}
