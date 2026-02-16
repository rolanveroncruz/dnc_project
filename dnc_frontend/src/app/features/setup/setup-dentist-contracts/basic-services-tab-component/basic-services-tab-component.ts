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
  /*
  Changes to make input controls appear as currency appear below:
   */
    private editingServiceId: number | null = null;

    displayValue(serviceId: number): string{
        const ctrl = this.ratesGroup?.get(this.controlName(serviceId));
        const v = ctrl?.value;

        if (v === null || v === undefined || v==='') return '';
        if (this.editingServiceId===serviceId) return String(v);
        const n =typeof v=== 'number'? v: Number(String(v).replace(/,/g,''));
        if (!Number.isFinite(n)) return '';
        return n.toLocaleString(undefined, { minimumFractionDigits:2, maximumFractionDigits:2});
    }
    onFocus(serviceId: number){
        this.editingServiceId = serviceId;
    }
    onBlur(serviceId: number, raw: string){
        this.editingServiceId = null;
        const ctrl = this.ratesGroup.get(this.controlName(serviceId));
        if (!ctrl) return;
        const cleaned = raw.replace(/,/g,'').trim();

        if (cleaned === ''){
            ctrl.setValue(null);
            ctrl.markAsTouched();
            return;
        }
        const n = Number(cleaned);
        if (!Number.isFinite(n)){
            ctrl.markAsTouched();
            return;
        }

        const normalized = Math.round(n*100)/100;
        ctrl.setValue(normalized);
        ctrl.markAsTouched();
    }

    protected readonly HTMLInputElement = HTMLInputElement;
}
