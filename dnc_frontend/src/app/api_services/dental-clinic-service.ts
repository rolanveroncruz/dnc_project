// src/app/api_services/dental-clinic-service.ts
import { Injectable, inject } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '../../environments/environment';

export interface PageResponse<T> {
  page: number;
  page_size: number;
  total_items: number;
  total_pages: number;
  items: T[];
}

// Matches your DentalClinicRow (including expanded fields)
export interface DentalClinicRow {
  id: number;
  name: string;
  address: string;
  city_id: number | null;
  remarks: string | null;
  contact_numbers: string | null;
  active: boolean | null;
  last_modified_by: string;
  last_modified_on: string; // ISO string from backend

  // expanded fields
  city_name: string | null;
  state_id: number | null;
  state_name: string | null;
  region_id: number | null;
  region_name: string | null;
}

export interface CreateDentalClinicRequest {
  name: string;
  address: string;
  city_id?: number | null;
  remarks?: string | null;
  contact_numbers?: string | null;
  active?: boolean | null;
}

export interface PatchDentalClinicRequest {
  name?: string | null;
  address?: string | null;
  city_id?: number | null;
  remarks?: string | null;
  contact_numbers?: string | null;
  active?: boolean | null;
}

@Injectable({ providedIn: 'root' })
export class DentalClinicService {
  private readonly http = inject(HttpClient);

  // backend routes:
  // GET    /api/dental-clinics?page=&page_size=&city_id=&state_id=&region_id=&active=
  // GET    /api/dental-clinics/:id
  // POST   /api/dental-clinics
  // PATCH  /api/dental-clinics/:id
  private readonly baseUrl = `${environment.apiUrl}/dental-clinics`;

  getDentalClinics(opts?: {
    page?: number;
    pageSize?: number;
    cityId?: number;
    stateId?: number;
    regionId?: number;
    active?: boolean;
  }): Observable<PageResponse<DentalClinicRow>> {
    let params = new HttpParams();

    if (opts?.page != null) params = params.set('page', String(opts.page));
    if (opts?.pageSize != null) params = params.set('page_size', String(opts.pageSize));

    if (opts?.cityId != null) params = params.set('city_id', String(opts.cityId));
    if (opts?.stateId != null) params = params.set('state_id', String(opts.stateId));
    if (opts?.regionId != null) params = params.set('region_id', String(opts.regionId));
    if (opts?.active != null) params = params.set('active', String(opts.active));

    return this.http.get<PageResponse<DentalClinicRow>>(this.baseUrl, { params });
  }

  getDentalClinicById(id: number): Observable<DentalClinicRow> {
    return this.http.get<DentalClinicRow>(`${this.baseUrl}/${id}`);
  }

  createDentalClinic(payload: CreateDentalClinicRequest): Observable<DentalClinicRow> {
    return this.http.post<DentalClinicRow>(this.baseUrl, payload);
  }

  patchDentalClinic(id: number, payload: PatchDentalClinicRequest): Observable<DentalClinicRow> {
    return this.http.patch<DentalClinicRow>(`${this.baseUrl}/${id}`, payload);
  }
}
