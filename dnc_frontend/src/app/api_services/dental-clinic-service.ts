// src/app/api_services/dental-clinic-service.ts
import { Injectable, inject } from '@angular/core';
import {HttpClient, HttpHeaders, HttpParams} from '@angular/common/http';
import {catchError, Observable, throwError} from 'rxjs';

// Adjust this import path to your project
import { environment } from '../../environments/environment';
import {LoginService} from '../login.service';

//
// ---- Types that mirror your Rust structs
//

export interface ListQuery {
  page?: number;       // 1-based
  page_size?: number;  // server clamps
}

export interface PageResponse<T> {
  page: number;       // 1-based
  page_size: number;
  total: number;
  items: T[];
}

export interface DentalClinic {
  id: number;
  name: string;
  address: string;
  city_id: number | null;
  zip_code: string | null;
  remarks: string | null;
  contact_numbers: string | null;
  email: string | null;
  schedule: string | null;
  active: boolean | null;
  last_modified_by: string;
  last_modified_on: string; // ISO datetime string (DateTimeWithTimeZone)
}

export interface DentalClinicListQuery extends ListQuery {
  city_id?: number;
  active?: boolean;
  name_like?: string;
}

export interface CreateDentalClinicBody {
  name: string;
  address: string;
  city_id?: number | null;
  city_name?: string | null;
  province_id?: number | null;
  province_name?: string | null;
  region_id?: number | null;
  region_name?: string | null;
  zip_code?: string | null;
  remarks?: string | null;
  contact_numbers?: string | null;
  email?: string | null;
  schedule?: string | null;
  active?: boolean | null;
  last_modified_by: string; // required by your API
}

/**
 * PATCH semantics:
 * - Omit a field => don't change
 * - Include field as null => explicitly set to null (for nullable columns)
 * - Include field as value => set to that value
 *
 * This maps nicely to your Rust `Option<Option<T>>`.
 */
export interface PatchDentalClinicBody {
  name?: string;
  address?: string;

  city_id?: number | null;
  zip_code?: string | null;
  remarks?: string | null;
  contact_numbers?: string | null;
  email?: string | null;
  schedule?: string | null;
  active?: boolean | null;

  last_modified_by: string; // required by your API
}

@Injectable({ providedIn: 'root' })
export class DentalClinicService {
  private readonly http = inject(HttpClient);
  private readonly loginService = inject(LoginService);

  // Example: environment.apiBaseUrl = 'http://localhost:3000'
  private readonly baseUrl = environment.apiUrl;

  // Adjust if your backend uses a different path
  private readonly API_PATH = '/api/dental_clinics';
  private authHeaders(): HttpHeaders {
    const token = this.loginService.token?.() ?? '';
    return new HttpHeaders({ Authorization: `Bearer ${token}` });
  }
  private handleError(err: unknown) {
    return throwError(() => err);
  }

  //
  // ---- GET: list (paged)
  //
  getDentalClinics(params: DentalClinicListQuery = {}): Observable<PageResponse<DentalClinic>> {
    const url = `${this.baseUrl}${this.API_PATH}/`;

    let httpParams = new HttpParams();

    if (params.page != null) httpParams = httpParams.set('page', String(params.page));
    if (params.page_size != null) httpParams = httpParams.set('page_size', String(params.page_size));

    if (params.city_id != null) httpParams = httpParams.set('city_id', String(params.city_id));
    if (params.active != null) httpParams = httpParams.set('active', String(params.active));
    if (params.name_like != null && params.name_like.trim().length > 0) {
      httpParams = httpParams.set('name_like', params.name_like.trim());
    }

    return this.http.get<PageResponse<DentalClinic>>(url, { params: httpParams, headers: this.authHeaders() })
      .pipe(catchError(this.handleError));
  }

  //
  // ---- GET: by id
  //
  getDentalClinicById(id: number): Observable<DentalClinic> {
    const url = `${this.baseUrl}${this.API_PATH}/${encodeURIComponent(id)}`;
    return this.http.get<DentalClinic>(url, { headers: this.authHeaders() })
      .pipe(catchError(this.handleError));
  }

  //
  // ---- POST: create
  //
  createDentalClinic(body: CreateDentalClinicBody): Observable<DentalClinic> {
    const url = `${this.baseUrl}${this.API_PATH}`;
    return this.http.post<DentalClinic>(url, body);
  }

  //
  // ---- PATCH: partial update
  //
  patchDentalClinic(id: number, body: PatchDentalClinicBody): Observable<DentalClinic> {
    const url = `${this.baseUrl}${this.API_PATH}/${encodeURIComponent(id)}`;
    return this.http.patch<DentalClinic>(url, body);
  }
}
