import {Component, DestroyRef, inject, OnInit, signal} from '@angular/core';
import { Router } from '@angular/router';
import { MatCard, MatCardContent, MatCardHeader, MatCardSubtitle, MatCardTitle } from '@angular/material/card';

import { GenericDataTableComponent } from '../../../components/generic-data-table-component/generic-data-table-component';
import { TableColumn } from '../../../components/generic-data-table-component/table-interfaces';
import {DentalClinicService} from '../../../api_services/dental-clinic-service';
// If you have a shared PageResponse/ListQuery type, import them; otherwise keep as `any`.
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';
import {MatButton} from '@angular/material/button';

export type DentalClinicRow = {
  id: number;
  name: string;
  address: string;
  city_id?: number | null;
  // Optional “joined” display fields if your API returns them
  city_name?: string | null;
  state_name?: string | null;
  region_name?: string | null;

  contact_numbers?: string | null;
  active?: boolean | null;

  last_modified_by?: string;
  last_modified_on?: string; // or DateTime string
  hasPanoramic?: boolean;
  hasPeriapical?: boolean;
};

@Component({
  selector: 'app-setup-dental-clinic',
  standalone: true,
  imports: [
    MatCard, MatCardHeader, MatCardContent, MatCardTitle, MatCardSubtitle,
    GenericDataTableComponent, MatButton,
  ],
  templateUrl: './setup-dental-clinics-component.html',
  styleUrl: './setup-dental-clinics-component.scss',
})
export class SetupDentalClinicsComponent implements OnInit{
  ngOnInit(): void {
    this.loadFn({});
  }
  private readonly router = inject(Router);
  private readonly destroyRef = inject(DestroyRef);
  private readonly dentalClinicsService = inject(DentalClinicService);
  dentalClinics = signal<DentalClinicRow[] | null>(null);

  // If your GenericDataTable expects a `columns` array, keep it like this.
  readonly columns: TableColumn<DentalClinicRow>[] = [
    { key: 'id', label: 'ID' },
    { key: 'name', label: 'Name' },
    { key: 'address', label: 'Address' },
    { key: 'city_name', label: 'City' },
    { key: 'province_name', label: 'Province' },
    { key: 'region_name', label: 'Region' },
    { key: 'contact_numbers', label: 'Contact' },
    { key: 'hasPanoramic', label: 'Panoramic Radio', cellTemplateKey: 'check'},
    { key: 'hasPeriapical', label: 'Periapical Radio', cellTemplateKey: 'check'},
  ];

  /**
   * Many of your other pages likely pass a function like this to the table.
   * Replace `any` with your real ListQuery / PageResponse types if you have them.
   */
  readonly loadFn = (params: any): void => {
    this.dentalClinicsService.getDentalClinics(params).pipe(
      takeUntilDestroyed(this.destroyRef),
    ).subscribe({
      next: res => {
        console.log(`clinics: ${res.items}`);
        this.dentalClinics.set(res.items)
      },
      error: err => console.error(err),
    });
  };



  onRowClicked(row: DentalClinicRow) {
    // Route example: /setup/dental-clinics/:id
    this.router.navigate(['/main/setup/dental-clinics', row.id]).then();
  }

  onNewClinic() {
    this.router.navigate(['/main/setup/dental-clinics/new']).then();
  }
}
